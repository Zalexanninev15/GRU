use execute::Execute;
use std::fs;
use std::process;
use std::process::Command;

// Get current working directory
pub fn current_dir() -> String {
    let mut current_dir = String::from(format!(
        "{}\\",
        std::env::current_dir().unwrap().display().to_string()
    ));
    if current_dir.contains("UpdateTools") == false {
        current_dir.push_str("UpdateTools\\");
    }
    return current_dir;
}

// Run script after updating application
pub fn run_post_script(current_dir: &str) {
    let script_file = String::from(format!("{}\\script.bat", current_dir));

    let output = Command::new("cmd")
        .args(&["/C", &script_file])
        .output()
        .expect("failed to execute process");

    for out in String::from_utf8(output.stdout).iter() {
        println!("{}", out);
    }
}

// Kill application processes
pub fn task_kill(application_exe: &str) -> std::io::Result<()> {
    const TASKKILL_TOOL: &str = "taskkill";
    let mut command = Command::new(TASKKILL_TOOL);

    command.arg("/F").arg("/T").arg("/IM").arg(application_exe);

    let output = command.execute_output().unwrap();
    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("The application processes are killed. (this is good)");
        } else {
            eprintln!("The process was not found. (this is good)");
        }
    } else {
        eprintln!("Interrupted!");
        press_btn_continue::wait("Press any key to exit...").unwrap();
        process::exit(1);
    }
    Ok(())
}

// Downloading github release by redl
pub fn downloading_by_redl(github_repo_path: &str, part: &str) {
    const EGET_PATH: &str = "redl.exe";
    let mut command = Command::new(EGET_PATH);
    // let download_path = String::from(format!("{}app.dat", current_dir));
    if part.contains(" ") {
        command
            .arg("-r")
            .arg(github_repo_path)
            .arg("-p")
            .args(part.split(" "))
            .arg("-o")
            .arg("app.dat");
    } else {
        command
            .arg("-r")
            .arg(github_repo_path)
            .arg("-p")
            .arg(part)
            .arg("-o")
            .arg("app.dat");
    }

    let output = command.execute_output().unwrap();
    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Downloaded.");
        } else {
            eprintln!("Failed.");
            press_btn_continue::wait("Press any key to exit...").unwrap();
            process::exit(1);
        }
    } else {
        eprintln!("Interrupted!");
        press_btn_continue::wait("Press any key to exit...").unwrap();
        process::exit(1);
    }
}

// Update by rename file
pub fn updating(current_dir: &str, launcher_exe: &str) -> std::io::Result<()> {
    fs::rename(
        String::from(format!("{}\\app.dat", current_dir)),
        String::from(format!("{}\\..\\{}", current_dir, launcher_exe)),
    )?;
    Ok(())
}

// Extract from EXE file of the portable installer
pub fn extracting(current_dir: &str) {
    const ZIPTOOL_PATH: &str = "7z.exe";
    let mut command = Command::new(ZIPTOOL_PATH);
    let extract_to = String::from(format!("-o{}..\\", current_dir));
    let exreact_file = String::from(format!("{}\\app.dat", current_dir));

    command
        .arg("x")
        .arg(exreact_file)
        .arg(extract_to)
        .arg("-r")
        .arg("-aoa")
        .arg("-bso0");

    let output = command.execute_output().unwrap();
    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Extracted.");
        } else {
            eprintln!("Failed.");
            press_btn_continue::wait("Press any key to exit...").unwrap();
            process::exit(1);
        }
    } else {
        eprintln!("Interrupted!");
        press_btn_continue::wait("Press any key to exit...").unwrap();
        process::exit(1);
    }
}

// Delete portable installer
pub fn delete_file(current_dir: &str, is_leave_folders: &bool) -> std::io::Result<()> {
    let file_dir = String::from(format!("{}app.dat", current_dir));
    fs::remove_file(file_dir)?;
    if (!is_leave_folders) {
        let dir_dir = String::from(format!("{}..\\$PLUGINSDIR", current_dir));
        fs::remove_dir_all(dir_dir)?;
    }
    let file_dir_zip = String::from(format!("{}*.zip", current_dir));
    fs::remove_file(file_dir_zip)?;
    let file_dir_7z = String::from(format!("{}*.7z", current_dir));
    fs::remove_file(file_dir_7z)?;
    let file_dir_rar = String::from(format!("{}*.rar", current_dir));
    fs::remove_file(file_dir_rar)?;
    Ok(())
}
