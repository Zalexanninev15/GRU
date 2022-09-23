use std::path::Path;
use std::process;

mod downloader;
mod get_version;
mod json;
mod main_func;
mod windows;

fn main() {
    let arguments = std::env::args();
    let current_dir = main_func::current_dir();
    let mut first_launch = false;
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut update_now = true;
    if arguments.len() >= 3 {
        let arguments = arguments::parse(arguments)
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let repo = arguments
            .get::<String>("repo")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let launcher_exe = arguments
            .get::<String>("app")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let part = arguments
            .get::<String>("with")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let real_app_path_bin = arguments
            .get::<String>("main")
            .unwrap_or(launcher_exe.to_string().parse().unwrap());

        // TODO: Working directory
        // let work_dir = arguments
        //     .get::<String>("path")
        //     .unwrap_or("\\..\\".to_string());

        let is_zip = arguments.get::<bool>("extract").unwrap_or(true);
        let is_leave_folders = arguments.get::<bool>("leave").unwrap_or(false);
        let is_script_after = arguments.get::<bool>("script").unwrap_or(false);
        let silent_mode = arguments.get::<bool>("silent").unwrap_or(false);
        winconsole::console::set_title("Github Release Updater")
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        println!(
            "Github Release Updater v{} by Zalexanninev15 <blue.shark@disroot.org>",
            VERSION
        );
        if windows::is_app_elevated() {
            // Checking the Internet connection
            let ic = main_func::test_iconnection();
            if ic.is_err() {
                println!("Error connecting to GitHub!");
                update_now = false;
            }

            let app_path = format!("{}\\..\\{}", current_dir, real_app_path_bin).to_string();

            // Is this the first download?
            if Path::new("app.version").exists() == false || Path::new(&app_path).exists() == false
            {
                first_launch = true;
            }

            // Getting the new version release
            let (v_list_version, mut v_list_asset) = json::parse_data(&repo, &part);

            // Delete the hash-files from string
            v_list_asset = v_list_asset
                .replace(".sha256sum", "")
                .replace(".md5sum", "")
                .replace(".md5", "")
                .replace(".MD5", "")
                .replace(".sha256", "")
                .replace(".SHA256", "")
                .replace(".sha-1", "")
                .replace(".SHA-1", "")
                .replace(".sha-1sum", "")
                .replace(".sha1", "")
                .replace(".SHA1", "")
                .replace(".hash", "")
                .replace(".HASH", "")
                .to_string();

            // Checker for PE version and new version
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
                if first_launch {
                    println!("Adding file(s)...");
                } else {
                    println!("Updating...");
                }
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
                if !silent_mode {
                    if first_launch {
                        main_func::set_new_version(&v_list_version);
                        press_btn_continue::wait("Download completed successfully!");
                    } else {
                        main_func::set_new_version(&v_list_version);
                        press_btn_continue::wait("Upgrade completed successfully!");
                    }
                } else {
                    if first_launch {
                        main_func::set_new_version(&v_list_version);
                        println!("Download completed successfully!");
                    } else {
                        main_func::set_new_version(&v_list_version);
                        println!("Upgrade completed successfully!");
                    }
                }
                process::exit(0);
            }
        } else {
            press_btn_continue::wait("Administrator rights are required to run!").unwrap();
            process::exit(1);
        }
    } else {
        const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
        println!("Github Release Updater
Description: {}
Version: v{}
Developer: Zalexanninev15 <blue.shark@disroot.org>
License: MIT License
GitHub: https://github.com/Zalexanninev15/GRU\n
USAGE:
    gru.exe --repo <user/repository> --app <application.exe> --with <value for search>\n
ARGUMENTS:
    --repo <user/repository> — Set the repository of application
    --app <application.exe> — Set the EXE of launcher/main application. The executable file must be located in a folder at a higher level, otherwise you need to set the '--main' argument with the correct path to the file
    --with <value for search> — Set the part of name of asset in GitHub release for download, for example: \"win-amd64-portable.zip\"\n
OPTIONAL:
    --main <target> - Set the main part of the application, the path to the application located at the level above [Default value = value of the '--app' argument]
    {{extract value}} → --extract or --no-extract — Set the type of file, extract archivers (flag) or copy EXE of launcher/main application [Default value: --extract]
    {{leave value}} → --leave or --no-leave - Not delete or delete the unnecessary folders: $PLUGINSDIR, Other [Default value: --no-leave]
    {{script value}} → --script or --no-script — Run the script (file \"script.bat\") after downloading the application or not [Default value: --no-script]
    {{pause value}} → --silent or --no-silent — Hide the console after work or not [Default value: --no-silent]\n
EXAMPLES:
    gru.exe --repo gek64/GitHubDesktopPortable --app GitHubDesktopPortable.exe --with \"paf\" --main App\\GitHubDesktop\\GitHubDesktop.exe
    gru.exe --repo flameshot-org/flameshot --app flameshot.exe --with \"win64.zip\" --script
    gru.exe --repo jgraph/drawio-desktop --app app.exe --with \"-windows-32bit-no-installer.exe\" --no-extract\n", DESCRIPTION, VERSION);
    }
    // --rv <value> — Set the executable from which you want to take the current version of the application. If you don't know exactly where to take it from, put any value, e.g. \"0\"\n
    press_btn_continue::wait("Press Enter to exit...").unwrap();
}
