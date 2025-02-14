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
        println!("Error: \"downloadtool.cfg\" not found.");
        process::exit(1);
    }

    "curl"
}

// Writing version information to file
pub fn set_new_version(version: &str) {
    let path = "app.version";
    let mut file = File::create(path).expect("Error creating file with version information!");
    file.write_all(version.as_bytes()).expect("Error writing version information to file!");
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
        .expect("failed to execute process");
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
            eprintln!("Failed.");
            press_btn_continue::wait("Press any key to exit...").unwrap();
            process::exit(1);
        }
    } else {
        eprintln!("Interrupted!");
        press_btn_continue::wait("Press any key to exit...").unwrap();
        process::exit(1);
    }

    if *is_nuget {
        let lib_path = Path::new(&extract_to).join("lib");
        if !lib_path.exists() {
            eprintln!("Failed: No lib directory found in NuGet package");
            press_btn_continue::wait("Press any key to exit...").unwrap();
            process::exit(1);
        }

        let framework_versions = vec!["net481", "net48", "net471", "net47", "net46", "net45"];
        let target_framework = match
            framework_versions.iter().find(|&version| lib_path.join(version).exists())
        {
            Some(version) => version,
            None => {
                eprintln!("Failed: No supported .NET framework version found");
                press_btn_continue::wait("Press any key to exit...").unwrap();
                process::exit(1);
            }
        };

        let source_path = lib_path.join(target_framework);
        let temp_dir = format!("{}..\\temp_nuget\\", current_dir);

        // Delete files from nuget package
        if let Err(_) = fs::remove_file(format!("{}..\\[Content_Types].xml", current_dir)) {
            eprintln!("Failed to clean up '[Content_Types].xml'");
        }
        if let Err(_) = fs::remove_dir_all(format!("{}..\\_rels", current_dir)) {
            eprintln!("Failed to clean up '_rels' directory");
        }

        // Safely create temp directory
        if let Err(_) = fs::create_dir_all(&temp_dir) {
            eprintln!("Failed to create temporary directory");
            press_btn_continue::wait("Press any key to exit...").unwrap();
            process::exit(1);
        }

        // Copy framework files to temp
        if let Err(_) = copy_directory(&source_path, Path::new(&temp_dir)) {
            eprintln!("Failed to copy framework files to temporary directory");
            press_btn_continue::wait("Press any key to exit...").unwrap();
            process::exit(1);
        }

        // Only remove the lib directory
        if let Err(_) = fs::remove_dir_all(lib_path) {
            eprintln!("Failed to clean up lib directory");
            press_btn_continue::wait("Press any key to exit...").unwrap();
            process::exit(1);
        }

        // Move files from temp to destination
        if let Err(_) = copy_directory(Path::new(&temp_dir), Path::new(&extract_to)) {
            eprintln!("Failed to move files to final location");
            press_btn_continue::wait("Press any key to exit...").unwrap();
            process::exit(1);
        }

        // Clean up temp directory
        if let Err(_) = fs::remove_dir_all(&temp_dir) {
            eprintln!("Warning: Failed to clean up temporary directory");
        }

        println!("Extracted from {} framework directory.", target_framework);
    } else {
        println!("Extracted.");
    }
}

fn copy_directory(source: &Path, destination: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            let dest_path = destination.join(file_name);
            fs::copy(path, dest_path)?;
        }
    }
    Ok(())
}

// Delete portable installer
pub fn delete_file(current_dir: &str, is_leave_folders: &bool) {
    let file_dir = String::from(format!("{}\\app.downloaded", current_dir));
    if Path::new(&file_dir).exists() {
        fs::remove_file(file_dir).expect("Temporary file \"app.downloaded\" not found.");
    }
    if !is_leave_folders {
        let mut dir = format!("{}..\\$PLUGINSDIR", current_dir).to_string();
        if Path::new(&dir).exists() {
            fs::remove_dir_all(dir).expect("Unnecessary folder \"$PLUGINSDIR\" was not found.");
        }
        dir = format!("{}..\\Other", current_dir).to_string();
        if Path::new(&dir).exists() {
            fs::remove_dir_all(dir).expect("Unnecessary folder \"Other\" was not found.");
        }
    }
}
