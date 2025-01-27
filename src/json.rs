use isahc::ReadResponseExt;
use serde_json::Value;

pub fn parse_data(repo: &str, search_words: &str, pre: bool) -> (String, String) {
    if !pre {
        return fetch_and_parse_release(
            &format!("https://api.github.com/repos/{}/releases/latest", repo),
            search_words
        );
    }

    // When pre=true, fetch all releases at once instead of making multiple API calls
    let releases = fetch_and_parse_releases(
        &format!("https://api.github.com/repos/{}/releases", repo)
    );

    let releases_array = match releases.as_array() {
        Some(array) if !array.is_empty() => array,
        _ => {
            return fetch_and_parse_release(
                &format!("https://api.github.com/repos/{}/releases/latest", repo),
                search_words
            );
        }
    };

    // Find latest stable and pre-release
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
}

fn fetch_and_parse_release(url: &str, search_words: &str) -> (String, String) {
    let json = isahc
        ::get(url)
        .expect("GitHub API: Error 404")
        .text()
        .expect("GitHub API: Json lost");
    parse_text(&json, search_words)
}

fn fetch_and_parse_releases(url: &str) -> Value {
    let json = isahc
        ::get(url)
        .expect("GitHub API: Error 404")
        .text()
        .expect("GitHub API: Json lost");
    serde_json::from_str(&json).expect("GitHub API: Error parsing json")
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

        // Break early if we found both
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
