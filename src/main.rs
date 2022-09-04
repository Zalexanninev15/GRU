use std::process;

mod downloader;
mod get_version;
mod json;
mod main_func;
mod windows;

fn main() {
    let arguments = std::env::args();
    let current_dir = main_func::current_dir();
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut update_now = true;
    if arguments.len() >= 8 {
        let arguments = arguments::parse(arguments).unwrap();
        let repo = arguments.get::<String>("repo").unwrap();
        let is_zip = arguments.get::<bool>("extract").unwrap();
        let launcher_exe = arguments.get::<String>("app").unwrap();
        let part = arguments.get::<String>("with").unwrap();
        let is_leave_folders = arguments.get::<bool>("leave").unwrap();
        let real_app_name_bin_with_path = arguments.get::<String>("rv").unwrap();
        let is_script_after = arguments.get::<bool>("script").unwrap();
        let is_pause = arguments.get::<bool>("pause").unwrap();
        winconsole::console::set_title("Github Releases Updater").unwrap();
        println!(
            "Github Releases Updater v{} by Zalexanninev15 <blue.shark@disroot.org>",
            VERSION
        );
        if windows::is_app_elevated() {
            // Getting the new version release
            let (v_list_version, v_list_asset) = json::parse_data(&repo, &part);

            if real_app_name_bin_with_path != "0" {
                // Checker for PE version and new version
                let app_path = format!("{}\\..\\{}", current_dir, real_app_name_bin_with_path);
                let version_status_code = get_version::is_new_version(&v_list_version, &app_path);
                if version_status_code != 1 {
                    println!("New version ({}) is available!", v_list_version);
                    if version_status_code == -1 {
                        println!("However, it may be inaccurate, since. the original version was not correctly defined!")
                    }
                } else {
                    update_now = false;
                    press_btn_continue::wait("No updates detected!");
                }
            }
            if update_now {
                main_func::task_kill(&launcher_exe);
                main_func::delete_file(&current_dir, &is_leave_folders);
                println!("Downloading...");

                // Old downloader by redl
                // main_func::downloading_by_redl(&repo, &part);

                // New native downloader
                downloader::download(&repo, &v_list_version, &v_list_asset, &current_dir);

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
            }
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
