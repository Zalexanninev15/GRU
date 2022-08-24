use execute::Execute;
use std::fs;
use std::process;
use std::process::Command;

mod windows;

fn main() {
    let arguments = std::env::args();
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    if arguments.len() >= 6 {
        let arguments = arguments::parse(arguments).unwrap();
        let repo = arguments.get::<String>("repo").unwrap();
        let is_zip = arguments.get::<bool>("extract").unwrap();
        let launcher_exe = arguments.get::<String>("app").unwrap();
        let part = arguments.get::<String>("with").unwrap();
        let is_leave_folders = arguments.get::<bool>("leave").unwrap();
        let is_script_after = arguments.get::<bool>("script").unwrap();
        let is_pause = arguments.get::<bool>("pause").unwrap();
        winconsole::console::set_title("Github Releases Updater").unwrap();
        println!(
            "Github Releases Updater v{} by Zalexanninev15 <blue.shark@disroot.org>",
            VERSION
        );
        if windows::is_app_elevated() {
            let current_dir = current_dir();
            task_kill(&launcher_exe);
            delete_file(&current_dir, &is_leave_folders);
            println!("Downloading...");
            downloading_by_redl(&repo, &part);
            if is_zip {
                println!("Extracting...");
                extracting(&current_dir);
            } else {
                println!("Updating...");
                updating(&current_dir, &launcher_exe);
            }
            // Delete the EXE file of the portable installer
            delete_file(&current_dir, &is_leave_folders);
            if is_script_after {
                println!("Running script.bat...");
                run_post_script(&current_dir);
            }
            if is_pause {
                press_btn_continue::wait("Update completed successfully!");
            } else {
                println!("Update completed successfully!");
            }
            process::exit(0);
        } else {
            press_btn_continue::wait("Administrator rights are required to run!").unwrap();
            process::exit(1);
        }
    } else {
        const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
        println!("Github Releases Updater
Description: {}
Version: v{}
Developer: Zalexanninev15 <blue.shark@disroot.org>
License: MIT License
GitHub: https://github.com/Zalexanninev15/GRU\n
USAGE:
    gru.exe --repo {{user/repository}} {{extract value}} --app {{application.exe}} --with {{value}} {{leave folders value}} {{script value}} {{pause value}}\n
ARGUMENTS:
    --repo {{user/repository}} — Set the repository of application
    {{extract value}} → --extract or --no-extract — Set the type of file, extract archivers (flag) or copy exe of launcher/main app
    --app {{application.exe}} — Set the exe of launcher/main application
    --with {{value}} — Set the part of name of asset in GitHub release for download (several parts of the name can be used, as long as they are separated by a space and enclosed in quotation marks, for example: \"win amd64 portable\")
    {{leave folders value}} → --leave or --no-leave - Not delete or delete the unnecessary folders: $PLUGINSDIR
    {{script value}} → --script or --no-script — Run script or not after update of application (file \"script.bat\")
    {{pause value}} → --pause or --no-pause — Set pause on finish of update\n
EXAMPLES:
    gru.exe --repo gek64/GitHubDesktopPortable --extract --app GitHubDesktopPortable.exe --with \"paf\" --no-leave --no-script --pause
    gru.exe --repo flameshot-org/flameshot --extract --app flameshot.exe --with \"win64.zip\" --no-leave --script --pause\n", DESCRIPTION, VERSION);
    }
    press_btn_continue::wait("Press Enter to exit...").unwrap();
}

// Get current working directory
fn current_dir() -> String {
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
fn run_post_script(current_dir: &str) {
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
fn task_kill(application_exe: &str) -> std::io::Result<()> {
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
fn downloading_by_redl(github_repo_path: &str, part: &str) {
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
fn updating(current_dir: &str, launcher_exe: &str) -> std::io::Result<()> {
    fs::rename(
        String::from(format!("{}\\app.dat", current_dir)),
        String::from(format!("{}\\..\\{}", current_dir, launcher_exe)),
    )?;
    Ok(())
}

// Extract from EXE file of the portable installer
fn extracting(current_dir: &str) {
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
fn delete_file(current_dir: &str, is_leave_folders: &bool) -> std::io::Result<()> {
    let file_dir = String::from(format!("{}app.dat", current_dir));
    fs::remove_file(file_dir)?;
    if (!is_leave_folders) {
        let dir_dir = String::from(format!("{}..\\$PLUGINSDIR", current_dir));
        fs::remove_dir_all(dir_dir)?;
    }
    Ok(())
}
