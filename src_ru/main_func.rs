use execute::Execute;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;
use std::io::{ self, BufRead };

pub fn read_downloadtool_config() -> &'static str {
    let path = Path::new(r"..\..\..\Scripts\downloadtool.cfg");

    if path.exists() {
        if let Ok(file) = File::open(&path) {
            let mut lines = io::BufReader::new(file).lines();
            if let Some(Ok(line)) = lines.next() {
                return Box::leak(line.into_boxed_str());
            }
        }
    } else {
        println!("Ошибка: 'downloadtool.cfg' не найден.");
        process::exit(1);
    }

    "curl"
}

// Writing version information to file
pub fn set_new_version(version: &str) {
    let path = "app.version";
    let mut file = File::create(path).expect("Ошибка при создании файла с информацией о версии!");
    file.write_all(version.as_bytes()).expect("Ошибка при записи информации о версии в файл!");
}

// Checking the Internet connection
pub fn test_iconnection() -> Result<(), isahc::Error> {
    isahc::get("https://github.com")?;
    Ok(())
}

// Get current working directory
pub fn current_dir() -> String {
    let mut current_dir = String::from(
        format!("{}\\", std::env::current_dir().unwrap().display().to_string())
    );
    if !current_dir.contains("UpdateTools") {
        current_dir.push_str("UpdateTools\\");
    }
    return current_dir;
}

// Run script after updating application
pub fn run_script(current_dir: &str, before: &bool) {
    let script_file = if *before {
        String::from(format!("{}\\prepare.bat", current_dir))
    } else {
        String::from(format!("{}\\script.bat", current_dir))
    };

    let script = Command::new("cmd")
        .args(&["/C", &script_file])
        .output()
        .expect("Ошибка создания процесса!");
    for out in String::from_utf8(script.stdout).iter() {
        println!("{}", out);
    }
}

// Kill application processes
pub fn task_kill(application_exe: &str, manifest_exists: &bool) {
    let killer = if *manifest_exists { "taskkill" } else { "tskill" };

    let mut command = Command::new(killer);

    if *manifest_exists {
        command.args(&["/F", "/T", "/IM", application_exe]);
    } else {
        command.arg(application_exe);
    }

    command.execute().unwrap();
}

// Update by rename file
pub fn updating(current_dir: &str, launcher_exe: &str) -> std::io::Result<()> {
    fs::rename(
        String::from(format!("{}\\app.downloaded", current_dir)),
        String::from(format!("{}\\..\\{}", current_dir, launcher_exe))
    )?;
    Ok(())
}

// Extract from EXE file of the portable installer or archive

pub fn extracting(current_dir: &str, is_nuget: &bool) {
    let mut command = Command::new("7z.exe");
    let extract_to = format!("{}..\\", current_dir);
    let extract_file = format!("{}\\app.downloaded", current_dir);

    command
        .arg("x")
        .arg(&extract_file)
        .arg(format!("-o{}", extract_to))
        .arg("-r")
        .arg("-aoa")
        .arg("-bso0");

    let output = command.execute_output().unwrap();
    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            eprintln!("Ошибка: Ошибка 7z!");
            press_btn_continue::wait("Нажмите Enter для выхода...").unwrap();
            process::exit(1);
        }
    } else {
        eprintln!("Ошибка: Операция распаковки не завершена!");
        press_btn_continue::wait("Нажмите Enter для выхода...").unwrap();
        process::exit(1);
    }

    if *is_nuget {
        let lib_path = Path::new(&extract_to).join("lib");
        if !lib_path.exists() {
            eprintln!("Ошибка: Не найдено папки 'lib' с исполнямемыми файлами!");
            press_btn_continue::wait("Нажмите Enter для выхода...").unwrap();
            process::exit(1);
        }

        let framework_versions = vec!["net481", "net48", "net471", "net47", "net46", "net45"];
        let target_framework = match
            framework_versions.iter().find(|&version| lib_path.join(version).exists())
        {
            Some(version) => version,
            None => {
                eprintln!("Ошибка: Не найдена поддерживаемая версия .NET Framework!");
                press_btn_continue::wait("Нажмите Enter для выхода...").unwrap();
                process::exit(1);
            }
        };

        let source_path = lib_path.join(target_framework);
        let temp_dir = format!("{}..\\temp_nuget\\", current_dir);

        // Delete files from nuget package
        if let Err(_) = fs::remove_file(format!("{}..\\[Content_Types].xml", current_dir)) {
            eprintln!("Не удалось удалить файл '[Content_Types].xml'");
        }
        if let Err(_) = fs::remove_dir_all(format!("{}..\\_rels", current_dir)) {
            eprintln!("Не удалось очистить каталог '_rels'");
        }

        // Safely create temp directory
        if let Err(_) = fs::create_dir_all(&temp_dir) {
            eprintln!("Не удалось создать временную папку!");
            press_btn_continue::wait("Нажмите Enter для выхода...").unwrap();
            process::exit(1);
        }

        // Copy framework files to temp
        if let Err(_) = copy_directory(&source_path, Path::new(&temp_dir)) {
            eprintln!("Не удалось скопировать файлы во временный каталог!");
            press_btn_continue::wait("Нажмите Enter для выхода...").unwrap();
            process::exit(1);
        }

        // Only remove the lib directory
        if let Err(_) = fs::remove_dir_all(lib_path) {
            eprintln!("Не удалось очистить каталог 'lib'!");
            press_btn_continue::wait("Нажмите Enter для выхода...").unwrap();
            process::exit(1);
        }

        // Move files from temp to destination
        if let Err(_) = copy_directory(Path::new(&temp_dir), Path::new(&extract_to)) {
            eprintln!("Не удалось переместить файлы в необходимую папку!");
            press_btn_continue::wait("Нажмите Enter для выхода...").unwrap();
            process::exit(1);
        }

        // Clean up temp directory
        if let Err(_) = fs::remove_dir_all(&temp_dir) {
            eprintln!("Предупреждение: Не удалось очистить временный каталог!");
        }

        println!("Обнаружен: {}!", target_framework);
    } else {
        println!("Распаковано.");
    }
}

fn copy_directory(source: &Path, destination: &Path) -> io::Result<()> {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let dest_path = destination.join(file_name);

        if path.is_file() {
            fs::copy(&path, &dest_path)?;
        } else if path.is_dir() {
            copy_directory(&path, &dest_path)?;
        }
    }

    Ok(())
}

// Delete portable installer
pub fn delete_file(current_dir: &str, is_leave_folders: &bool) {
    let file_dir = String::from(format!("{}\\app.downloaded", current_dir));
    if Path::new(&file_dir).exists() {
        fs::remove_file(file_dir).expect("Нет временного файла \"app.downloaded\".");
    }
    if !is_leave_folders {
        let mut dir = format!("{}..\\$PLUGINSDIR", current_dir).to_string();
        if Path::new(&dir).exists() {
            fs::remove_dir_all(dir).expect("Нет папки \"$PLUGINSDIR\".");
        }
        dir = format!("{}..\\Other", current_dir).to_string();
        if Path::new(&dir).exists() {
            fs::remove_dir_all(dir).expect("Нет папки \"Other\".");
        }
    }
}
