use std::fs::File;
use std::path::Path;
use std::process::{ Command, Stdio };
use trauma::{
    download::Download,
    downloader::{ DownloaderBuilder, StyleOptions, ProgressBarOpts },
};
use std::{ fs, path::PathBuf };
use console::style;
use std::io::{ self, Read };

use crate::main_func;

// Download the asset
pub fn download(
    repo: &str,
    ver_tag: &str,
    file: &str,
    details: &bool,
    mut downloader: &str,
    ua: &str,
    use_cfg: &bool
) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = crate::main_func::current_dir();
    let file_name = String::from(format!("{}\\app.downloaded", current_dir));
    let gru1 = if downloader == "gru-classic" {
        downloader = "gru";
        true
    } else {
        false
    };
    let mut asset = String::from(repo);
    if repo.contains("://") == false {
        let formatted_asset = format!(
            "https://github.com/{}/releases/download/{}/{}",
            repo,
            ver_tag,
            file
        );
        asset = formatted_asset;
    }

    if downloader == "tcpud" {
        let _ = execute_tcpu_download_script(
            main_func::read_downloadtool_config(),
            &asset,
            &current_dir
        );
    } else {
        match downloader {
            "curl" => {
                let mut command = Command::new("C:\\Windows\\System32\\curl.exe");

                if Path::new(&String::from(format!("{}\\curl.txt", current_dir))).exists() {
                    let mut file = File::open(
                        String::from(format!("{}\\curl.txt", current_dir))
                    ).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    command = Command::new(contents.replace("\"", "").replace("/", "\\"));
                } else {
                    if Path::new(&String::from(format!("{}\\curl.exe", current_dir))).exists() {
                        command = Command::new(String::from(format!("{}\\curl.exe", current_dir)));
                    } else {
                        if
                            Path::new(
                                &String::from(format!("C:\\Windows\\System32\\curl.exe"))
                            ).exists()
                        {
                            command = Command::new("C:\\Windows\\System32\\curl.exe");
                        } else {
                            downloader = "gru";
                            let rt = tokio::runtime::Runtime::new().unwrap();
                            let _ = rt.block_on(native(&current_dir, &file, &asset, &gru1));
                        }
                    }
                }

                if downloader == "curl" {
                    command
                        .arg("-L")
                        .arg("-o")
                        .arg(file_name)
                        .arg("-A")
                        .arg(String::from(format!("\"{}\"", ua)))
                        .arg(asset);
                    if *details == false {
                        command.arg("--progress-bar");
                    }
                    command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
                    execute_command(command);
                }
            }
            "wget" => {
                let mut cfg_path = String::new();

                let mut command = Command::new("C:\\Windows\\System32\\wget.exe");

                if Path::new(&String::from(format!("{}\\wget.txt", current_dir))).exists() {
                    let mut file = File::open(
                        String::from(format!("{}\\wget.txt", current_dir))
                    ).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    let wget_exe = contents.replace("\"", "").replace("/", "\\");
                    if *use_cfg {
                        cfg_path = if Path::new(&format!("{}\\.wgetrc", current_dir)).exists() {
                            format!("{}\\.wgetrc", current_dir)
                        } else {
                            wget_exe.replace("wget.exe", ".wgetrc")
                        };
                    }
                    command = Command::new(wget_exe);
                } else {
                    if Path::new(&format!("{}\\wget.exe", current_dir)).exists() {
                        if *use_cfg {
                            if Path::new(&format!("{}\\.wgetrc", current_dir)).exists() {
                                cfg_path = format!("{}\\.wgetrc", current_dir);
                            }
                        }
                        command = Command::new(String::from(format!("{}\\wget.exe", current_dir)));
                    } else {
                        downloader = "gru";
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let _ = rt.block_on(native(&current_dir, &file, &asset, &gru1));
                    }
                }
                if downloader == "wget" {
                    if *use_cfg {
                        command
                            .arg(asset)
                            .arg(String::from(format!("-U=\"{}\"", ua)))
                            .arg("-O")
                            .arg(file_name);
                        if !cfg_path.is_empty() {
                            command.arg(format!("--config={}", cfg_path));
                        }
                    } else {
                        command
                            .arg(String::from(format!("-U=\"{}\"", ua)))
                            .arg("--tries=2")
                            .arg("--no-check-certificate")
                            .arg("--no-cache");
                        if *details {
                            command.arg("-O").arg(file_name).arg(asset);
                        } else {
                            command
                                .arg("-q")
                                .arg("-O")
                                .arg(file_name)
                                .arg(asset)
                                .arg("--show-progress=on");
                        }
                    }
                    command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
                    execute_command(command);
                }
            }
            "gru" => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let _ = rt.block_on(native(&current_dir, &file, &asset, &gru1));
            }
            _ => {}
        }
    }
    Ok(())
}

fn execute_tcpu_download_script(
    mut tool_name: &str,
    asset: &str,
    application_path: &str
) -> io::Result<()> {
    if tool_name != "curl" {
        tool_name = "curl";
    }
    let script_path: PathBuf = ["..", "..", "..", "Scripts", &format!("download_{}.bat", tool_name)]
        .iter()
        .collect();
    Command::new("cmd")
        .args(
            &[
                "/C",
                script_path.to_str().unwrap(),
                asset,
                &format!("{}app.downloaded", application_path),
            ]
        )
        .current_dir(std::env::current_dir()?)
        .spawn()?
        .wait()?;
    Ok(())
}

fn execute_command(mut command: Command) {
    let mut child = command.spawn().expect("Не удалось запустить сторонний загрузчик!");

    let status = child.wait().expect("Преждевременное завершение процесса!");

    if !status.success() {
        eprintln!("Команда выполнена с кодом ошибки: {}", status);
    }
}

async fn native(
    application_path: &str,
    file: &str,
    asset: &str,
    is_classic_progressbar: &bool
) -> std::io::Result<()> {
    let d_file = String::from(format!("{}\\{}", application_path, file));
    if Path::new(&d_file).exists() {
        fs::remove_file(&d_file).expect("\n");
    }
    let downloads = vec![Download::try_from(asset).unwrap()];
    let style_opts = if *is_classic_progressbar {
        StyleOptions::default()
    } else {
        StyleOptions::new(
            ProgressBarOpts::new(
                Some(ProgressBarOpts::TEMPLATE_BAR_WITH_POSITION.into()),
                Some(ProgressBarOpts::CHARS_FINE.into()),
                true,
                false
            ),
            ProgressBarOpts::new(
                Some(
                    format!(
                        "{{bar:40.cyan}} {{bytes_per_sec:>13.green}} {{percent:>2.cyan}}{} {{bytes:>11.green}}/{{total_bytes:<11.green}}",
                        style("%").cyan()
                    )
                ),
                Some("████ ".into()),
                true,
                false
            )
        )
    };
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from(application_path))
        .style_options(style_opts)
        .build();
    downloader.download(&downloads).await;
    fs::rename(&d_file, String::from(format!("{}\\app.downloaded", application_path)))?;
    Ok(())
}
