use std::{fs, path::PathBuf};
use tokio;
use trauma::{
    download::Download,
    downloader::{DownloaderBuilder, StyleOptions},
    Error,
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
    let downloads = vec![Download::try_from(asset).unwrap()];
    let style_opts = StyleOptions::default();
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from(application_path))
        .style_options(style_opts)
        .build();
    downloader.download(&downloads).await;
    fs::rename(
        String::from(format!("{}\\{}", application_path, file)),
        String::from(format!("{}\\app.dat", application_path)),
    )?;
    Ok(())
}
