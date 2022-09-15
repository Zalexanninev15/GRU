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
        let arguments = arguments::parse(arguments)
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let repo = arguments
            .get::<String>("repo")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let is_zip = arguments
            .get::<bool>("extract")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let launcher_exe = arguments
            .get::<String>("app")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let part = arguments
            .get::<String>("with")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let is_leave_folders = arguments
            .get::<bool>("leave")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let real_app_name_bin = arguments
            .get::<String>("rv")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let is_script_after = arguments
            .get::<bool>("script")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let is_pause = arguments
            .get::<bool>("pause")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        winconsole::console::set_title("Github Releases Updater")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        println!(
            "Github Releases Updater v{} by Zalexanninev15 <blue.shark@disroot.org>",
            VERSION
        );
        if windows::is_app_elevated() {
            // Checking the Internet connection
            let ic = main_func::test_iconnection();
            if ic.is_err() {
                println!("Error connecting to GitHub!");
                update_now = false;
            }

            // Getting the new version release
            let (v_list_version, mut v_list_asset) = json::parse_data(&repo, &part);

            // Delete the hash-files from string
            v_list_asset = v_list_asset
                .replace(".sha256sum", "")
                .replace(".SHA256SUM", "")
                .replace(".md5sum", "")
                .replace(".MD5SUM", "")
                .replace(".md5", "")
                .replace(".MD5", "")
                .replace(".sha256", "")
                .replace(".SHA256", "")
                .replace(".sha-1", "")
                .replace(".SHA-1", "")
                .replace(".sha-1sum", "")
                .replace(".SHA-1SUM", "")
                .replace(".sha1sum", "")
                .replace(".SHA1SUM", "")
                .replace(".sha1", "")
                .replace(".SHA1", "")
                .replace(".hash", "")
                .replace(".HASH", "")
                .to_string();

            // Checker for PE version and new version
            let app_path = format!("{}\\..\\{}", current_dir, real_app_name_bin);
            let version_status_code = get_version::is_new_version(&v_list_version, &app_path);
            if version_status_code != 0 {
                println!("\nNew version ({}) is available!", v_list_version);
                if version_status_code == -1 {
                    println!("\nHowever, it may be inaccurate, since. the original version was not correctly defined!")
                }
            } else {
                update_now = false;
                println!("\nNo updates detected!");
            }

            // Updater
            if update_now {
                // Deleting unnecessary data
                main_func::task_kill(&launcher_exe);
                main_func::delete_file(&current_dir, &is_leave_folders);

                // Old downloader by redl
                // main_func::downloading_by_redl(&repo, &part);

                // New native downloader
                println!("Downloading...");
                let result =
                    downloader::download(&repo, &v_list_version, &v_list_asset, &current_dir);
                if result.is_err() {
                    println!("Failed to download!");
                }

                // The updating process itself
                println!("Updating...");
                if is_zip {
                    main_func::extracting(&current_dir);
                } else {
                    let ue = main_func::updating(&current_dir, &launcher_exe);
                    if ue.is_err() {
                        println!("File replacement error!")
                    }
                }

                // Delete the EXE file of the portable installer
                main_func::delete_file(&current_dir, &is_leave_folders);
                if is_script_after {
                    println!("Running script.bat...");
                    main_func::run_post_script(&current_dir);
                }

                // Should I pause the console after work or not?
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
    gru.exe --repo {{user/repository}} {{extract value}} --app {{application.exe}} --with {{value}} {{leave folders value}} --rv {{value}} {{script value}} {{pause value}}\n
ARGUMENTS:
    --repo {{user/repository}} — Set the repository of application
    {{extract value}} → --extract or --no-extract — Set the type of file, extract archivers (flag) or copy exe of launcher/main app
    --app {{application.exe}} — Set the exe of launcher/main application
    --with {{value}} — Set the part of name of asset in GitHub release for download (several parts of the name can be used, as long as they are separated by a space and enclosed in quotation marks, for example: \"win amd64 portable\")
    {{leave folders value}} → --leave or --no-leave - Not delete or delete the unnecessary folders: $PLUGINSDIR, Other
    --rv {{value}} — Set the executable from which you want to take the current version of the application. If you don't know exactly where to take it from, put any value, e.g. \"0\"
    {{script value}} → --script or --no-script — Run script or not after update of application (file \"script.bat\")
    {{pause value}} → --pause or --no-pause — Set pause on finish of update\n
EXAMPLES:
    gru.exe --repo gek64/GitHubDesktopPortable --extract --app GitHubDesktopPortable.exe --with \"paf\" --no-leave --rv App\\GitHubDesktop\\GitHubDesktop.exe --no-script --pause
    gru.exe --repo flameshot-org/flameshot --extract --app flameshot.exe --with \"win64.zip\" --no-leave --rv flameshot.exe --script --pause\n", DESCRIPTION, VERSION);
    }
    press_btn_continue::wait("Press Enter to exit...").unwrap();
}
