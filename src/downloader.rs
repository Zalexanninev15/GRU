use std::{fs, path::PathBuf};
use trauma::{
    download::Download,
    downloader::{DownloaderBuilder, StyleOptions},
    Error,
};

// Download the asset
pub fn download(
    repo: &str,
    ver_tag: &str,
    file: &str,
    application_path: &str,
) -> Result<(), Error> {
    let asset = &*format!(
        "https://github.com/{}/releases/download/{}/{}",
        repo, ver_tag, file
    )
    .to_string();
    let downloads = vec![Download::try_from(asset).unwrap()];
    let style_opts = StyleOptions::default();
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from(application_path))
        .style_options(style_opts)
        .build();
    downloader.download(&downloads);
    fs::rename(
        String::from(format!("{}\\{}", application_path, file)),
        String::from(format!("{}\\app.dat", application_path)),
    )?;
    Ok(())
}
