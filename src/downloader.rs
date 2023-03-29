use isahc::{HttpClient};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, io::Write};
use futures_lite::io::AsyncReadExt;

// Downloader V3
pub async fn download_file(
    repo: &str,
    ver_tag: &str,
    file: &str,
    application_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        repo, ver_tag, file
    );
    let client = HttpClient::new()?;
    let response = client.get_async(&url).await?;
    let total_size = response.headers().get("content-length")
        .and_then(|s| s.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap().progress_chars("#>-"));
    let mut source = response.into_body();
    let file_path = format!("{}\\{}", application_path, file);
    let mut dest = fs::File::create(&file_path)?;
    let mut buffer = [0; 1024 * 1024];
    loop {
        let n = source.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        dest.write_all(&buffer[..n])?;
        pb.inc(n as u64);
    }
    pb.finish_with_message("downloaded");
    fs::rename(file_path, format!("{}\\app.dat", application_path))?;
    Ok(())
}

// Old downloader
// #[tokio::main]
// pub async fn download(
//     repo: &str,
//     ver_tag: &str,
//     file: &str,
//     application_path: &str,
// ) -> Result<(), Error> {
//     // println!("Debug print: {} {} {} {}", repo, ver_tag, file, application_path);
//     let asset = &*String::from(format!(
//         "https://github.com/{}/releases/download/{}/{}",
//         repo, ver_tag, file
//     ));
//     let downloads = vec![Download::try_from(asset).unwrap()];
//     let style_opts = StyleOptions::default();
//     let downloader = DownloaderBuilder::new()
//         .directory(PathBuf::from(application_path))
//         .style_options(style_opts)
//         .build();
//     downloader.download(&downloads).await;
//     fs::rename(
//         String::from(format!("{}\\{}", application_path, file)),
//         String::from(format!("{}\\app.dat", application_path)),
//     )?;
//     Ok(())
// }
