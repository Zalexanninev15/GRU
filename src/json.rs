use isahc::ReadResponseExt;
use serde_json::Value;

pub fn parse_data(
    repo: &str,
    search_words: &str,
    pre: &bool,
    one_release: &bool
) -> (String, String) {
    if *pre == false {
        return fetch_and_parse_release(
            &format!("https://api.github.com/repos/{}/releases/latest", repo),
            search_words,
            one_release
        );
    }
    // When pre=true, fetch all releases at once
    let releases = fetch_and_parse_releases(
        &format!("https://api.github.com/repos/{}/releases", repo)
    );

    let releases_array = match releases.as_array() {
        Some(array) if !array.is_empty() => array,
        _ => {
            return fetch_and_parse_release(
                &format!("https://api.github.com/repos/{}/releases/latest", repo),
                search_words,
                one_release
            );
        }
    };

    if *one_release {
        // Original behavior: Find latest stable and pre-release
        let (stable, prerelease) = find_latest_releases(releases_array);

        // Determine which release to use based on dates
        let selected_release = match (stable, prerelease) {
            (Some(stable), Some(pre)) => {
                let pre_date = pre["published_at"].as_str().unwrap_or("");
                let stable_date = stable["published_at"].as_str().unwrap_or("");
                if pre_date > stable_date {
                    pre
                } else {
                    stable
                }
            }
            (Some(stable), None) => stable,
            (None, Some(pre)) => pre,
            (None, None) => {
                return (String::new(), String::from("app.zip"));
            }
        };

        parse_text(&serde_json::to_string(selected_release).unwrap(), search_words)
    } else {
        // New behavior: Search through multiple releases
        for release in releases_array {
            if let Some(assets) = release["assets"].as_array() {
                for asset in assets {
                    if let Some(name) = asset["name"].as_str() {
                        if name.contains(search_words) {
                            return (
                                release["tag_name"].as_str().unwrap_or("").to_string(),
                                name.to_string(),
                            );
                        }
                    }
                }
            }
        }
        // If no matching asset is found in any release
        (String::new(), String::from("app.zip"))
    }
}

fn fetch_and_parse_release(url: &str, search_words: &str, one_release: &bool) -> (String, String) {
    if *one_release {
        let json = isahc
            ::get(url)
            .expect("GitHub API: Error 404")
            .text()
            .expect("GitHub API: Json lost");
        parse_text(&json, search_words)
    } else {
        // For multi-release search, we need to get all releases
        let releases_url = url.replace("/latest", "");
        let releases = fetch_and_parse_releases(&releases_url);

        if let Some(releases_array) = releases.as_array() {
            for release in releases_array {
                if let Some(assets) = release["assets"].as_array() {
                    for asset in assets {
                        if let Some(name) = asset["name"].as_str() {
                            if name.contains(search_words) {
                                return (
                                    release["tag_name"].as_str().unwrap_or("").to_string(),
                                    name.to_string(),
                                );
                            }
                        }
                    }
                }
            }
        }
        (String::new(), String::from("app.zip"))
    }
}

fn fetch_and_parse_releases(url: &str) -> Value {
    let json = isahc
        ::get(url)
        .expect("GitHub API: Error 404")
        .text()
        .expect("GitHub API: Json lost");
    serde_json::from_str(&json).expect("GitHub API: Error parsing json")
}

// Version 3.1 ???
/* extern crate reqwest;
use reqwest::header;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert("Authorization", "Bearer sssssss".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get("https://api.github.com/repos/obsproject/obs-studio/releases/latest")
        .headers(headers)
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
} */

fn find_latest_releases(releases: &[Value]) -> (Option<&Value>, Option<&Value>) {
    let mut latest_stable = None;
    let mut latest_prerelease = None;

    for release in releases {
        let is_prerelease = release["prerelease"].as_bool().unwrap_or(false);

        if is_prerelease && latest_prerelease.is_none() {
            latest_prerelease = Some(release);
        } else if !is_prerelease && latest_stable.is_none() {
            latest_stable = Some(release);
        }

        if latest_stable.is_some() && latest_prerelease.is_some() {
            break;
        }
    }

    (latest_stable, latest_prerelease)
}

fn parse_text(json: &str, word: &str) -> (String, String) {
    let release: Value = serde_json::from_str(json).expect("GitHub API: Error parsing json");
    let mut asset_name = String::from("app.zip");

    if let Some(assets) = release["assets"].as_array() {
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                if name.contains(word) {
                    asset_name = name.to_string();
                    break;
                }
            }
        }
    }

    (release["tag_name"].as_str().unwrap_or("").to_string(), asset_name)
}
