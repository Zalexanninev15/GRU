use execute::Execute;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;

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
    let mut current_dir = String::from(format!(
        "{}\\",
        std::env::current_dir().unwrap().display().to_string()
    ));
    if !current_dir.contains("UpdateTools") {
        current_dir.push_str("UpdateTools\\");
    }
    return current_dir;
}

// Run script after updating application
pub fn run_post_script(current_dir: &str) {
    let script_file = String::from(format!("{}\\script.bat", current_dir));
    let script = Command::new("cmd")
        .args(&["/C", &script_file])
        .output()
        .expect("failed to execute process");
    for out in String::from_utf8(script.stdout).iter() {
        println!("{}", out);
    }

    // script.arg(format!("/C {}", &script_file));
    // script.execute_output().unwrap();
}

// Kill application processes
pub fn task_kill(application_exe: &str) {
    const TASKKILL_TOOL: &str = "taskkill";
    let mut command = Command::new(TASKKILL_TOOL);

    command.arg("/F").arg("/T").arg("/IM").arg(application_exe);
    command.execute();
    // let output = command.execute_output().unwrap();
    // if let Some(exit_code) = output.status.code() {
    //     if exit_code == 0 {
    //         println!("The application processes are killed. (this is good)");
    //     } else {
    //         eprintln!("The process was not found. (this is good)");
    //     }
    // } else {
    //     eprintln!("Interrupted!");
    //     press_btn_continue::wait("Press any key to exit...").unwrap();
    //     process::exit(1);
    // }
}

// Downloading github release by redl
// pub fn downloading_by_redl(github_repo_path: &str, part: &str) {
//     const EGET_PATH: &str = "redl.exe";
//     let mut command = Command::new(EGET_PATH);
//     // let download_path = String::from(format!("{}app.dat", current_dir));
//     if part.contains(" ") {
//         command
//             .arg("-r")
//             .arg(github_repo_path)
//             .arg("-p")
//             .args(part.split(" "))
//             .arg("-o")
//             .arg("app.dat");
//     } else {
//         command
//             .arg("-r")
//             .arg(github_repo_path)
//             .arg("-p")
//             .arg(part)
//             .arg("-o")
//             .arg("app.dat");
//     }

//     let output = command.execute_output().unwrap();
//     if let Some(exit_code) = output.status.code() {
//         if exit_code == 0 {
//             println!("Downloaded.");
//         } else {
//             eprintln!("Failed.");
//             press_btn_continue::wait("Press any key to exit...").unwrap();
//             process::exit(1);
//         }
//     } else {
//         eprintln!("Interrupted!");
//         press_btn_continue::wait("Press any key to exit...").unwrap();
//         process::exit(1);
//     }
// }

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
pub fn delete_file(current_dir: &str, is_leave_folders: &bool) {
    let file_dir = String::from(format!("{}app.dat", current_dir));
    if Path::new(&file_dir).exists() {
        fs::remove_file(file_dir).expect("Temporary file \"app.dat\" not found.");
    }
    if !is_leave_folders {
        let mut dir  = format!("{}..\\$PLUGINSDIR", current_dir).to_string();
        if Path::new(&dir).exists() {
            fs::remove_dir_all(dir)
                .expect("Unnecessary folder \"$PLUGINSDIR\" was not found.");
        }
        dir  = format!("{}..\\Other", current_dir).to_string();
        if Path::new(&dir).exists() {
            fs::remove_dir_all(dir)
                .expect("Unnecessary folder \"Other\" was not found.");
        }
    }
}
