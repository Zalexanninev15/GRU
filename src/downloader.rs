use std::{fs, path::{PathBuf, Path}};
use console::style;
use tokio;
use trauma::{
    download::Download,
    downloader::{DownloaderBuilder, StyleOptions, ProgressBarOpts},
    Error
};

// Download the asset
#[tokio::main]
pub async fn download(
    repo: &str,
    ver_tag: &str,
    file: &str,
    application_path: &str,
) -> Result<(), Error> {
    // println!("[DEBUG] {} {} {} {}", repo, ver_tag, file, application_path);
    let asset = &*String::from(format!(
        "https://github.com/{}/releases/download/{}/{}",
        repo, ver_tag, file
    ));
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
            false,
        ),
        ProgressBarOpts::new(
            Some(format!(
                "{{bar:40.cyan}} {{bytes_per_sec:>13.green}} {{percent:>2.cyan}}{} {{bytes:>11.green}}/{{total_bytes:<11.green}}",
                style("%").cyan(),
            )),
            Some("████ ".into()),
            true,
            false,
        )
    );
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from(application_path))
        .style_options(style_opts)
        .build();
    downloader.download(&downloads).await;
    fs::rename(
        &d_file,
        String::from(format!("{}\\app.dat", application_path)),
    )?;
    Ok(())
}