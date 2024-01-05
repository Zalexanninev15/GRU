use std::process::{Command, Stdio};
use std::process;


// Download the asset

// #[tokio::main]
// pub async fn download(
pub fn download(repo: &str, ver_tag: &str, file: &str, simple_mode: &bool) {
    let asset = &*String::from(
        format!("https://github.com/{}/releases/download/{}/{}", repo, ver_tag, file)
    );

    let current_dir = crate::main_func::current_dir();
    let file_name = String::from(format!("{}\\app.dat", current_dir));

    let mut command = Command::new("C:\\Windows\\System32\\curl.exe");

    if *simple_mode {
        command
        .arg("-Lo")
        .arg(file_name)
        .arg(asset)
        .arg("--progress-bar")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    }
    else {
        command
        .arg("-Lo")
        .arg(file_name)
        .arg(asset)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    }

    let mut child = command.spawn().expect("Failed to start curl process");

    let status = child.wait().expect("Failed to wait for curl process");

    if !status.success() {
        eprintln!("Command executed with failing error code: {}", status);
        process::exit(1);
    }
    // let d_file = String::from(format!("{}\\{}", application_path, file));
    // if Path::new(&d_file).exists() {
    //     fs::remove_file(&d_file).expect("\n");
    // }
    // let downloads = vec![Download::try_from(asset).unwrap()];
    // let style_opts = StyleOptions::new(
    //     ProgressBarOpts::new(
    //         Some(ProgressBarOpts::TEMPLATE_BAR_WITH_POSITION.into()),
    //         Some(ProgressBarOpts::CHARS_FINE.into()),
    //         true,
    //         false,
    //     ),
    //     ProgressBarOpts::new(
    //         Some(format!(
    //             "{{bar:40.cyan}} {{bytes_per_sec:>13.green}} {{percent:>2.cyan}}{} {{bytes:>11.green}}/{{total_bytes:<11.green}}",
    //             style("%").cyan(),
    //         )),
    //         Some("████ ".into()),
    //         true,
    //         false,
    //     )
    // );
    // let downloader = DownloaderBuilder::new()
    //     .directory(PathBuf::from(application_path))
    //     .style_options(style_opts)
    //     .build();
    // downloader.download(&downloads).await;
    // fs::rename(
    //     &d_file,
    //     String::from(format!("{}\\app.dat", application_path)),
    // )?;
    // Ok(())
}
