use std::fs::File;
use std::path::Path;
use std::process::{ Command, Stdio };
use std::process;
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

    if downloader == "tcpu" {
        execute_tcpu_download_script(main_func::read_downloadtool_config(), &asset, &current_dir);
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
                    command = Command::new(contents.replace("\"", ""));
                } else {
                    if Path::new(&String::from(format!("{}\\curl.exe", current_dir))).exists() {
                        command = Command::new(String::from(format!("{}\\curl.exe", current_dir)));
                    } else {
                        downloader = "bn";
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let _ = rt.block_on(native(&current_dir, &file, &asset));
                    }
                }

                if downloader == "curl" {
                    if *details {
                        command
                            .arg("--user-agent")
                            .arg(String::from(format!("\"{}\"", ua)))
                            .arg(String::from(format!("-H \"User-Agent: {}\"", ua)))
                            .arg("-Lo")
                            .arg(file_name)
                            .arg(asset)
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit());
                    } else {
                        command
                            .arg("--user-agent")
                            .arg(String::from(format!("\"{}\"", ua)))
                            .arg(String::from(format!("-H \"User-Agent: {}\"", ua)))
                            .arg("-Lo")
                            .arg(file_name)
                            .arg(asset)
                            .arg("--progress-bar")
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit());
                    }

                    execute_command(command);
                }
            }
            "aria2c" => {
                let mut command = Command::new("C:\\Windows\\System32\\aria2c.exe");

                if Path::new(&String::from(format!("{}\\aria2c.txt", current_dir))).exists() {
                    let mut file = File::open(
                        String::from(format!("{}\\aria2c.txt", current_dir))
                    ).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    command = Command::new(contents.replace("\"", ""));
                } else {
                    if Path::new(&String::from(format!("{}\\aria2c.exe", current_dir))).exists() {
                        command = Command::new(
                            String::from(format!("{}\\aria2c.exe", current_dir))
                        );
                    } else {
                        downloader = "bn";
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let _ = rt.block_on(native(&current_dir, &file, &asset));
                    }
                }
                if downloader == "aria2c" {
                    if *details {
                        command
                            .arg(asset)
                            .arg(String::from(format!("--user-agent=\"{}\"", ua)))
                            .arg("--console-log-level=error")
                            .arg("--split=8")
                            .arg("--max-connection-per-server=16")
                            .arg("--min-split-size=4M")
                            .arg("--piece-length=4M")
                            .arg("--continue=false")
                            .arg("--remote-time=true")
                            .arg("--auto-file-renaming=false")
                            .arg("--allow-overwrite=true")
                            .arg("--connect-timeout=5")
                            .arg("--lowest-speed-limit=5K")
                            .arg("--max-tries=12")
                            .arg("--max-file-not-found=12")
                            .arg("--no-netrc=false")
                            .arg("--timeout=60")
                            .arg("--check-certificate=false")
                            .arg("--http-accept-gzip=true")
                            .arg("--http-no-cache=true")
                            .arg("--enable-http-keep-alive=true")
                            .arg("--allow-piece-length-change=false")
                            .arg("--conditional-get=true")
                            .arg("--disable-ipv6=true")
                            .arg("--disk-cache=4M")
                            .arg("--download-result=hide")
                            .arg("--file-allocation=falloc")
                            .arg("--summary-interval=0")
                            .arg("-o")
                            .arg(file_name);
                    } else {
                        command
                            .arg(asset)
                            .arg(String::from(format!("--user-agent=\"{}\"", ua)))
                            .arg("--console-log-level=error")
                            .arg("--split=8")
                            .arg("--max-connection-per-server=16")
                            .arg("--min-split-size=4M")
                            .arg("--piece-length=4M")
                            .arg("--continue=false")
                            .arg("--remote-time=true")
                            .arg("--auto-file-renaming=false")
                            .arg("--allow-overwrite=true")
                            .arg("--connect-timeout=5")
                            .arg("--lowest-speed-limit=5K")
                            .arg("--max-tries=12")
                            .arg("--max-file-not-found=12")
                            .arg("--no-netrc=false")
                            .arg("--timeout=60")
                            .arg("--check-certificate=false")
                            .arg("--http-accept-gzip=true")
                            .arg("--http-no-cache=true")
                            .arg("--enable-http-keep-alive=true")
                            .arg("--allow-piece-length-change=false")
                            .arg("--conditional-get=true")
                            .arg("--disable-ipv6=true")
                            .arg("--disk-cache=4M")
                            .arg("--download-result=hide")
                            .arg("--file-allocation=falloc")
                            .arg("--summary-interval=0")
                            .arg("-o")
                            .arg(file_name);
                    }

                    execute_command(command);
                }
            }
            "wget" => {
                let mut command = Command::new("C:\\Windows\\System32\\wget.exe");

                if Path::new(&String::from(format!("{}\\wget.txt", current_dir))).exists() {
                    let mut file = File::open(
                        String::from(format!("{}\\wget.txt", current_dir))
                    ).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    command = Command::new(contents.replace("\"", ""));
                } else {
                    if Path::new(&String::from(format!("{}\\wget.exe", current_dir))).exists() {
                        command = Command::new(String::from(format!("{}\\wget.exe", current_dir)));
                    } else {
                        downloader = "bn";
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let _ = rt.block_on(native(&current_dir, &file, &asset));
                    }
                }
                if downloader == "wget" {
                    if *details {
                        command
                            .arg(String::from(format!("-U=\"{}\"", ua)))
                            .arg("--tries=2")
                            .arg("--no-check-certificate")
                            .arg("--cache=off")
                            .arg("--header")
                            .arg(String::from(format!("\"{}\"", ua)))
                            .arg("-O")
                            .arg(file_name)
                            .arg(asset);
                    } else {
                        command
                            .arg(String::from(format!("-U=\"{}\"", ua)))
                            .arg("--tries=2")
                            .arg("--no-check-certificate")
                            .arg("--cache=off")
                            .arg("--header")
                            .arg(String::from(format!("\"{}\"", ua)))
                            .arg("-q")
                            .arg("-O")
                            .arg(file_name)
                            .arg(asset)
                            .arg("--show-progress");
                    }

                    execute_command(command);
                }
            }
            "bn" => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let _ = rt.block_on(native(&current_dir, &file, &asset));
            }
            _ => {}
        }
    }
    Ok(())
}

fn execute_tcpu_download_script(
    tool_name: &str,
    asset: &str,
    application_path: &str
) -> io::Result<()> {
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
    let mut child = command.spawn().expect("Failed to start download process!");

    let status = child.wait().expect("Failed to wait for process!");

    if !status.success() {
        eprintln!("Command executed with failing error code: {}", status);
        process::exit(1);
    }
}

async fn native(application_path: &str, file: &str, asset: &str) -> std::io::Result<()> {
    let d_file = String::from(format!("{}\\{}", application_path, file));
    if Path::new(&d_file).exists() {
        fs::remove_file(&d_file).expect("\n");
    }
    let downloads = vec![Download::try_from(asset).unwrap()];
    let style_opts = StyleOptions::new(
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
    );
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from(application_path))
        .style_options(style_opts)
        .build();
    downloader.download(&downloads).await;
    fs::rename(&d_file, String::from(format!("{}\\app.downloaded", application_path)))?;
    Ok(())
}
