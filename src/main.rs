use std::fs::metadata;
use std::path::Path;
use std::process;
use winconsole::console;

mod downloader;
mod get_version;
mod json;
mod main_func;

fn main() {
    let arguments = std::env::args();
    let current_dir = main_func::current_dir();
    let mut first_launch = false;
    let mut create_only_version_file = false;
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let _ = console::set_title("Github Release Updater");

    let mut update_now = true;
    if arguments.len() >= 3 {
        let arguments = arguments
            ::parse(arguments)
            .expect("Argument error! Check the arguments according to the \"help\" of the utility");
        let mut repo = arguments
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
        let is_extract = arguments.get::<bool>("extract").unwrap_or(true);
        let is_leave_folders = arguments.get::<bool>("leave").unwrap_or(false);
        let is_script_after = arguments.get::<bool>("script").unwrap_or(false);
        let silent_mode = arguments.get::<bool>("silent").unwrap_or(false);
        let details = arguments.get::<bool>("details").unwrap_or(false);
        let tool = arguments.get::<String>("tool").unwrap_or("curl".to_string());
        let d_link = arguments.get::<String>("link").unwrap_or("null".to_string());
        let ua = arguments
            .get::<String>("ua")
            .unwrap_or(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36".to_string()
            );
        let use_cfg = arguments.get::<bool>("config").unwrap_or(false);
        let debug_mode = arguments.get::<bool>("debug").unwrap_or(false);

        println!("Github Release Updater v{} by Zalexanninev15 <blue.shark@disroot.org>", VERSION);
        let s = main_func::test_iconnection();
        if s.is_err() {
            println!("Error connecting to GitHub!");
            update_now = false;
        }

        // Application path
        let app_path = format!("{}\\..\\{}", current_dir, real_app_path_bin).to_string();

        // Debug mode
        if debug_mode {
            println!("[Debug] repo = \"{}\"", repo);
            println!("[Debug] launcher_exe = \"{}\"", launcher_exe);
            println!("[Debug] part = \"{}\"", part);
            println!("[Debug] real_app_path_bin = \"{}\"", real_app_path_bin);
            println!("[Debug] is_extract = {}", is_extract);
            println!("[Debug] is_leave_folders = {}", is_leave_folders);
            println!("[Debug] is_script_after = {}", is_script_after);
            println!("[Debug] silent_mode = {}", silent_mode);
            println!("[Debug] app_path = \"{}\"", app_path.replace("\\\\", "\\"));
            println!("[Debug] details = {}", details);
            println!("[Debug] tool = \"{}\"", tool);
            println!("[Debug] d_link = {}", d_link);
            println!("[Debug] ua = \"{}\"", ua);
            println!("[Debug] debug_mode = true");
            press_btn_continue::wait("[Debug] Press Enter to continue...").unwrap();
        }

        // Is this the first download?
        if Path::new(&app_path).exists() {
            if Path::new("app.version").exists() == false {
                create_only_version_file = true;
            } else {
                first_launch = false;
            }
        } else {
            first_launch = true;
        }

        // Getting the new version release
        let (v_list_version, mut v_list_asset) = json::parse_data(&repo, &part);

        if debug_mode {
            println!("[Debug] v_list_version = \"{}\"", v_list_version);
            println!("[Debug] v_list_asset = \"{}\"", v_list_asset);
            press_btn_continue::wait("[Debug] Press Enter to continue...").unwrap();
        }

        // Delete the hash-files from string
        v_list_asset = v_list_asset
            .replace(".sha256sum", "")
            .replace(".md5sum", "")
            .replace(".md5", "")
            .replace(".MD5", "")
            .replace(".sha512", "")
            .replace(".SHA512", "")
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

        if debug_mode {
            println!("[Debug] v_list_asset (after hash(s) deletion) = \"{}\"", v_list_asset);
            press_btn_continue::wait("[Debug] Press Enter to continue...").unwrap();
        }

        // Checker for сurrent and new version
        if create_only_version_file {
            println!("\nCurrent version of app: {}", &v_list_version);
            main_func::set_new_version(&v_list_version);
            update_now = false;
            println!("\nNo updates detected!");
        } else {
            let version_status_code = get_version::is_new_version(&v_list_version, &app_path);
            if version_status_code != 0 && create_only_version_file == false {
                println!("\nNew version ({}) is available!", v_list_version);
                if version_status_code == -1 {
                    println!(
                        "\nHowever, it may be inaccurate, since. the original version was not correctly defined!"
                    );
                }
            } else {
                update_now = false;
                println!("\nNo updates detected!");
            }
        }

        // Updater
        if update_now {
            // Deleting unnecessary data
            main_func::task_kill(&launcher_exe);
            main_func::delete_file(&current_dir, &is_leave_folders);

            if debug_mode {
                println!("[Debug] State 1");
            }

            // Downloading the file
            println!("Downloading...");
            if v_list_asset.contains(&part) == false && d_link != "null" {
                repo = d_link;
            }
            let _ = downloader::download(
                &repo,
                &v_list_version,
                &v_list_asset,
                &details,
                &tool,
                &ua,
                &use_cfg
            );

            if debug_mode {
                println!("[Debug] State 2");
            }

            // Fix for native downloader
            if Path::new(&String::from(format!("{}\\download", &current_dir))).exists() {
                let _ = std::fs::rename(
                    String::from(format!("{}\\download", &current_dir)),
                    String::from(format!("{}\\app.downloaded", &current_dir))
                );
            }

            if
                let Ok(metadata) = metadata(
                    String::from(format!("{}\\app.downloaded", &current_dir))
                )
            {
                if metadata.is_file() {
                    if debug_mode {
                        println!("[Debug] State 3");
                    }
                    // The updating process itself
                    if first_launch {
                        println!("Adding file(s)...");
                    } else {
                        println!("Updating...");
                    }
                    if is_extract {
                        main_func::extracting(&current_dir);
                    } else {
                        let ue = main_func::updating(&current_dir, &launcher_exe);
                        if ue.is_err() {
                            println!("File replacement error!");
                        }
                    }

                    // Delete the EXE file of the portable installer
                    main_func::delete_file(&current_dir, &is_leave_folders);

                    if is_script_after {
                        println!("Running script.bat...");
                        main_func::run_post_script(&current_dir);
                    }

                    main_func::set_new_version(&v_list_version);
                    if first_launch {
                        println!("Download completed successfully!");
                    } else {
                        println!("Upgrade completed successfully!");
                    }
                } else {
                    println!("The file, for some reason, was not downloaded!");
                }
            }
        } else {
            println!("The file, for some reason, was not downloaded!");
        }
        if debug_mode {
            println!("[Debug] State 4");
        }
        if !silent_mode || debug_mode {
            press_btn_continue::wait("Press Enter to exit...").unwrap();
        }
        process::exit(0);
    } else {
        const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
        println!(
            "Github Release Updater\nDescription: {}\nVersion: v{}\nDeveloper: Zalexanninev15 <blue.shark@disroot.org>\nLicense: MIT License\nGitHub: https://github.com/Zalexanninev15/GRU\n
USAGE:
    gru.exe --repo <user/repository> --app <application.exe> --with <value for search>\n
ARGUMENTS:
    * --repo <user/repository> — Set the repository of application
    * --app <application.exe> — Set the EXE of launcher/main application.
    The executable file must be located in a folder at a higher level,
    otherwise you need to set the '--main' argument with the correct path to the file
    * --with <value for search> — Set the part of name of asset in GitHub release for download,
    for example: \"win-amd64-portable.zip\"\n
OPTIONAL:
    * --main <target> - Set the main part of the application,
    the path to the application located at the level above [Default value: value of the '--app' argument]
    * {{extract value}} → --extract or --no-extract — Set the type of file,
    extract archivers (flag) or copy EXE of launcher/main application or not [Default value: --extract]
    * {{leave value}} → --leave or --no-leave - Not delete or delete
    the unnecessary folders: \"$PLUGINSDIR\", \"Other\" or not [Default value: --no-leave]
    * {{script value}} → --script or --no-script — Run the script (file \"script.bat\") after
    downloading the application or not [Default value: --no-script]
    * {{pause value}} → --silent or --no-silent — Hide the console after work or not [Default value: --no-silent]
    * {{details value}} → --details or --no-details - Show more information when downloading (curl/wget) or not
    [Default value: --no-details]
    * --tool <type> - Select a tool to download the file (you can select \"curl\", \"wget\", \"aria2c\", \"bn\" (built-in downloader), \"tcpu\").
	By default, 'curl.exe' or 'wget.exe' or 'aria2c.exe' files are used for \"curl\", \"wget\" and \"aria2c\" respectively,
	in the path \"C:\\Windows\\System32\". If there are installed utilities, the path to 
	them (to executable files) can be specified in the files \"curl.txt\", \"wget.txt\", \"aria2c.txt\".
    If there is an error finding an executable file, the built-in file downloader will be invoked.
    [Default value: curl]
    * --link <url> - Sometimes releases may not contain file to download, but just be a place for a list of changes. 
    Set the download link. [Default value: null]
    * --ua <user-agent> - A user-agent to pretend to be a browser and try to download the file at a higher speed (can sometimes help). 
    The argument applies only to the “curl”, “wget”, “aria2c” tools. [Default value: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36]
    * {{config value}} → --config or --no-config - Loading a configuration file for all downloaders except the built-in one (this argument takes precedence over all others, so use only a fully configured configuration file (except for the user-agent)). The configuration file is \".curlrc\" or \".wgetrc\" or \"aria2.conf\" is always located next to the executable file of similar utilities.
    * {{debug value}} → --debug or --no-debug - Debug mode or not [Default value: --no-debug]\n
EXAMPLES:
    Here: https://github.com/Zalexanninev15/GRU#usage",
            DESCRIPTION,
            VERSION
        );
    }
    // --rv <value> — Set the executable from which you want to take the current version of the application. If you don't know exactly where to take it from, put any value, e.g. \"0\"\n
    press_btn_continue::wait("Press Enter to exit...").unwrap();
}
