use std::fs::metadata;
use std::path::Path;
use std::process;
use winconsole::console;

mod downloader;
mod get_version;
mod json;
mod main_func;

use std::ptr;
use winapi::um::securitybaseapi::AllocateAndInitializeSid;
use winapi::um::securitybaseapi::CheckTokenMembership;
use winapi::um::winnt::{
    SECURITY_BUILTIN_DOMAIN_RID,
    DOMAIN_ALIAS_RID_ADMINS,
    SID_IDENTIFIER_AUTHORITY,
};

fn is_admin() -> bool {
    unsafe {
        let mut authority = SID_IDENTIFIER_AUTHORITY {
            Value: [0, 0, 0, 0, 0, 5], // 5 is SECURITY_NT_AUTHORITY
        };
        let mut sid = ptr::null_mut();

        // Create a SID for the BUILTIN\Administrators group
        let success = AllocateAndInitializeSid(
            &mut authority,
            2,
            SECURITY_BUILTIN_DOMAIN_RID,
            DOMAIN_ALIAS_RID_ADMINS,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut sid
        );

        if success == 0 {
            return false;
        }

        let mut is_member = 0;
        // Check if the current token is a member of the admin SID
        let status = CheckTokenMembership(ptr::null_mut(), sid, &mut is_member);

        if status != 0 {
            is_member != 0
        } else {
            false
        }
    }
}

fn main() {
    let arguments = std::env::args();
    let current_dir = main_func::current_dir();
    let mut first_launch = false;
    let mut create_only_version_file = false;
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    let _ = console::set_title("Github Release Updater");
    let mut update_now = true;

    if
        Path::new(&String::from(format!("{}\\7z.exe", current_dir))).exists() == false ||
        Path::new(&String::from(format!("{}\\7z.dll", current_dir))).exists() == false
    {
        println!("7z.exe or 7z.dll not found!");
        press_btn_continue::wait("Press Enter to exit...").unwrap();
    }

    if arguments.len() >= 3 {
        let arguments = arguments
            ::parse(arguments)
            .expect("Argument error! Just launch the GRU for show help");
        let mut repo = arguments
            .get::<String>("repo")
            .expect("Argument error! Just launch the GRU for show help");
        let launcher_exe = arguments
            .get::<String>("app")
            .expect("Argument error! Just launch the GRU for show help");
        let part = arguments
            .get::<String>("with")
            .expect("Argument error! Just launch the GRU for show help");
        let real_app_path_bin = arguments
            .get::<String>("main")
            .unwrap_or(launcher_exe.to_string().parse().unwrap());
        let is_extract = arguments.get::<bool>("extract").unwrap_or(true);
        let is_leave_folders = arguments.get::<bool>("leave").unwrap_or(false);
        let is_script_before = arguments.get::<bool>("before").unwrap_or(false);
        let is_script_after = arguments.get::<bool>("script").unwrap_or(false);
        let silent_mode = arguments.get::<bool>("silent").unwrap_or(false);
        let details = arguments.get::<bool>("details").unwrap_or(false);
        let tool = arguments.get::<String>("tool").unwrap_or("gru".to_string());
        let d_link = arguments.get::<String>("link").unwrap_or("null".to_string());
        let mut no_ghost = arguments.get::<bool>("ghost").unwrap_or(false);
        let ua = arguments
            .get::<String>("ua")
            .unwrap_or(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36".to_string()
            );
        let wgetrc = arguments.get::<bool>("wgetrc").unwrap_or(false);
        let show_pre = arguments.get::<bool>("pre").unwrap_or(false);
        let debug_mode = arguments.get::<bool>("debug").unwrap_or(false);

        println!("Github Release Updater v{} by Zalexanninev15 <blue.shark@disroot.org>", VERSION);
        let s = main_func::test_iconnection();
        if s.is_err() {
            println!("Error connecting to GitHub!");
            update_now = false;
        }

        // Application path
        let app_path = format!("{}\\..\\{}", current_dir, real_app_path_bin).to_string();

        no_ghost = !no_ghost;

        let admin = is_admin();
        if debug_mode {
            println!("[DEBUG] is_admin = {}", admin);
            println!("[Debug] repo = \"{}\"", repo);
            println!("[Debug] launcher_exe = \"{}\"", launcher_exe);
            println!("[Debug] part = \"{}\"", part);
            println!("[Debug] real_app_path_bin = \"{}\"", real_app_path_bin);
            println!("[Debug] is_extract = {}", is_extract);
            println!("[Debug] is_leave_folders = {}", is_leave_folders);
            println!("[Debug] is_script_before = {}", is_script_before);
            println!("[Debug] is_script_after = {}", is_script_after);
            println!("[Debug] silent_mode = {}", silent_mode);
            println!("[Debug] app_path = \"{}\"", app_path.replace("\\\\", "\\"));
            println!("[Debug] details = {}", details);
            println!("[Debug] tool = \"{}\"", tool);
            println!("[Debug] d_link = {}", d_link);
            println!("[Debug] ua = \"{}\"", ua);
            println!("[Debug] wgetrc = {}", wgetrc);
            println!("[Debug] show_pre = {}", show_pre);
            println!("[Debug] no_ghost = {}", no_ghost);
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
        let (v_list_version, mut v_list_asset) = json::parse_data(
            &repo,
            &part,
            &show_pre,
            &no_ghost
        );

        if debug_mode {
            println!("\n[Debug] v_list_version = \"{}\"", v_list_version);
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
            println!("\n[Debug] v_list_asset (after hash(s) deletion) = \"{}\"", v_list_asset);
            press_btn_continue::wait("[Debug] Press Enter to continue...\n").unwrap();
        }

        // Checker for сurrent and new version
        if create_only_version_file {
            println!("\nCurrent version of app: {}", &v_list_version);
            main_func::set_new_version(&v_list_version);
            update_now = false;
            println!("\nYou have the latest version!");
        } else {
            let version_status_code = get_version::is_new_version(&v_list_version, &app_path);
            if version_status_code != 0 && create_only_version_file == false {
                if v_list_version != "" {
                    println!("\nNew version {} is available!", v_list_version);
                } else {
                    update_now = false;
                    println!(
                        "\nNo new versions were found. If you are sure that they exist, use '--debug' or '--pre'."
                    );
                }
                if version_status_code == -1 {
                    println!(
                        "\nHowever, this may be inaccurate because the version may have been determined incorrectly!"
                    );
                }
            } else {
                update_now = false;
                println!("\nYou have the latest version!");
            }
        }

        // Updater
        if update_now {
            if is_script_before {
                println!("Running prepare.bat...");
                main_func::run_script(&current_dir, &true);
            }

            // Deleting unnecessary data
            main_func::task_kill(&launcher_exe, &admin);
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
                &wgetrc
            );

            if debug_mode {
                press_btn_continue::wait("Press Enter to continue...").unwrap();
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
                        main_func::run_script(&current_dir, &false);
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
            println!("You don't need to download anything.");
            println!(
                "Or check the arguments if you are sure that the download should have started at 100%."
            );
        }
        if debug_mode {
            println!("[Debug] State 4");
        }
        if !silent_mode || debug_mode {
            press_btn_continue::wait("Press Enter to exit...").unwrap();
        }
        process::exit(0);
    } else {
        println!(
            "Github Release Updater
Description: {}
Version: v{}
Developer: Zalexanninev15 <blue.shark@disroot.org>
License: MIT License
GitHub: https://github.com/Zalexanninev15/GRU

USAGE:
    gru.exe --repo <user/repository> --app <application.exe> --with <search_value>

ARGUMENTS:
    --repo <user/repository>      Specify the repository (e.g., 'user/repo').
    --app <application.exe>       Specify the main application executable.
                                  The executable should be in a higher-level folder.
                                  Use '--main' if located elsewhere.
    --with <search_value>         Specify a part of the asset name in the GitHub release
                                  to download (e.g., 'win-amd64-portable.zip').

OPTIONS:
    --main <path>                 Path to the main application. Defaults to '--app' value.
    --extract / --no-extract      Extract archive files or just copy the EXE. Default: --extract.
    --leave / --no-leave          Keep or delete unnecessary folders (e.g., $PLUGINSDIR). Default: --no-leave.
    --before / --no-before        Run 'prepare.bat' before download. Default: --no-before.
    --script / --no-script        Run 'script.bat' after download and extraction (or move). Default: --no-script.
    --silent / --no-silent        Hide console after execution. Default: --no-silent.
    --details / --no-details      Show detailed download information ('curl', 'wget'). Default: --no-details.
    --tool <type>                 File downloader tool ('curl', 'wget', 'gru', 'tcpud'). 
                                  Default: 'gru'.
    --link <url>                  Direct download URL if release lacks assets. Default: null.
    --ua <user-agent>             Specify a user-agent for better download speed.
                                  Default: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36.
    --wgetrc / --no-wgetrc        Use config file for 'wget' (.wgetrc). Default: --no-wgetrc.
    --pre / --no-pre              Use a pre-release instead of a stable release (if there are no stable releases or the unstable release was released after the stable release and is the most recent).
                                  Default: --no-pre.
    --ghost / --no-ghost          Search for matching assets across multiple recent releases instead of only the latest one. Default: --no-ghost.                                  
    --debug / --no-debug          Enable debug mode. Default: --no-debug.

EXAMPLES:
    Detailed examples available at: 
    https://github.com/Zalexanninev15/GRU#usage",
            DESCRIPTION,
            VERSION
        );
    }
    press_btn_continue::wait("Press Enter to exit...").unwrap();
}
