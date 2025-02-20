use isahc::prelude::*;
use isahc::{ Request, ReadResponseExt };
use serde_json::Value;

pub fn parse_data(
    repo: &str,
    search_words: &str,
    pre: &bool,
    one_release: &bool,
    ua: &str,
    api_key: Option<&str>
) -> (String, String) {
    if *pre == false {
        return fetch_and_parse_release(
            &format!("https://api.github.com/repos/{}/releases/latest", repo),
            search_words,
            one_release,
            ua,
            api_key
        );
    }

    let releases = fetch_and_parse_releases(
        &format!("https://api.github.com/repos/{}/releases", repo),
        ua,
        api_key
    );

    let releases_array = match releases.as_array() {
        Some(array) if !array.is_empty() => array,
        _ => {
            return fetch_and_parse_release(
                &format!("https://api.github.com/repos/{}/releases/latest", repo),
                search_words,
                one_release,
                ua,
                api_key
            );
        }
    };

    if *one_release {
        let (stable, prerelease) = find_latest_releases(releases_array);

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
        (String::new(), String::from("app.zip"))
    }
}

fn create_github_request(url: &str, ua: &str, api_key: Option<&str>) -> Request<()> {
    let mut builder = Request::builder().uri(url).header("User-Agent", format!("{}", ua));

    if let Some(key) = api_key {
        builder = builder
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", key))
            .header("X-GitHub-Api-Version", "2022-11-28");
    }

    builder.body(()).expect("GitHub API: Failed to create request!")
}

fn fetch_and_parse_release(
    url: &str,
    search_words: &str,
    one_release: &bool,
    ua: &str,
    api_key: Option<&str>
) -> (String, String) {
    if *one_release {
        let request = create_github_request(url, ua, api_key);
        let json = request
            .send()
            .expect("GitHub API: Error 404")
            .text()
            .expect("GitHub API: Json lost");
        parse_text(&json, search_words)
    } else {
        let releases_url = url.replace("/latest", "");
        let releases = fetch_and_parse_releases(&releases_url, ua, api_key);

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

fn fetch_and_parse_releases(url: &str, ua: &str, api_key: Option<&str>) -> Value {
    let request = create_github_request(url, ua, api_key);
    let json = request
        .send()
        .expect("GitHub API: Error 404!")
        .text()
        .expect("GitHub API: Json lost!");
    serde_json::from_str(&json).expect("GitHub API: Error parsing json!")
}

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
    let release: Value = serde_json::from_str(json).expect("GitHub API: Error parsing json!");
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
