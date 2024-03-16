use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::{ Command, Stdio };
use std::process;
use trauma::{
    download::Download,
    downloader::{ DownloaderBuilder, StyleOptions, ProgressBarOpts },
};
use std::{ fs, path::PathBuf };
use console::style;

// Download the asset
pub fn download(
    repo: &str,
    ver_tag: &str,
    file: &str,
    details: &bool,
    downloader: &str
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
            }

            if Path::new(&String::from(format!("{}\\curl.exe", current_dir))).exists() {
                command = Command::new(String::from(format!("{}\\curl.exe", current_dir)));
            }

            if *details {
                command
                    .arg("-Lo")
                    .arg(file_name)
                    .arg(asset)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());
            } else {
                command
                    .arg("-Lo")
                    .arg(file_name)
                    .arg(asset)
                    .arg("--progress-bar")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());
            }

            execute_command(command);
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
            }

            if Path::new(&String::from(format!("{}\\wget.exe", current_dir))).exists() {
                command = Command::new(String::from(format!("{}\\wget.exe", current_dir)));
            }

            if *details {
                command.arg("-O").arg(file_name).arg(asset);
            } else {
                command.arg("-q").arg("-O").arg(file_name).arg(asset).arg("--show-progress");
            }

            execute_command(command);
        }
        "native" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let _ = rt.block_on(native(&current_dir, &file, &asset));
        }
        _ => {}
    }

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
