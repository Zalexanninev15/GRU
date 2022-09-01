use std::process;

mod downloader;
mod get_version;
mod main_func;
mod windows;

fn main() {
    let arguments = std::env::args();
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    if arguments.len() >= 7 {
        let arguments = arguments::parse(arguments).unwrap();
        let repo = arguments.get::<String>("repo").unwrap();
        let is_zip = arguments.get::<bool>("extract").unwrap();
        let launcher_exe = arguments.get::<String>("app").unwrap();
        let part = arguments.get::<String>("with").unwrap();
        let is_leave_folders = arguments.get::<bool>("leave").unwrap();
        // let check_version_file_info = arguments.get::<String>("fv").unwrap();
        let is_script_after = arguments.get::<bool>("script").unwrap();
        let is_pause = arguments.get::<bool>("pause").unwrap();
        winconsole::console::set_title("Github Releases Updater").unwrap();
        println!(
            "Github Releases Updater v{} by Zalexanninev15 <blue.shark@disroot.org>",
            VERSION
        );

        // if (check_version_file_info != "0") {
        // Getting the new version release

        // Json parser

        // Checker for PE version and new version
        //     get_version::is_new_version();
        // }
        if windows::is_app_elevated() {
            let current_dir = main_func::current_dir();
            main_func::task_kill(&launcher_exe);
            main_func::delete_file(&current_dir, &is_leave_folders);
            println!("Downloading...");
            main_func::downloading_by_redl(&repo, &part);
            if is_zip {
                println!("Extracting...");
                main_func::extracting(&current_dir);
            } else {
                println!("Updating...");
                main_func::updating(&current_dir, &launcher_exe);
            }
            // Delete the EXE file of the portable installer
            main_func::delete_file(&current_dir, &is_leave_folders);
            if is_script_after {
                println!("Running script.bat...");
                main_func::run_post_script(&current_dir);
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
