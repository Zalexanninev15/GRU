use isahc::ReadResponseExt;
use serde_json::Value;

// Main function of parser
pub fn parse_data(repo: &str, search_words: &str) -> (String, String) {
    let json = isahc
        ::get(String::from(format!("https://api.github.com/repos/{}/releases/latest", repo)))
        .expect("GitHub API: Error 404")
        .text()
        .expect("GitHub API: Json lost");
    let release_data = parse_text(&json, &search_words);
    release_data
}

// Parse json with release data
fn parse_text(json: &str, word: &str) -> (String, String) {
    let release: Value = serde_json::from_str(json).expect("GitHub API: Error parsing json");
    let mut asset_name = String::from("app.zip");
    for rs in release["assets"].as_array().unwrap() {
        let name = rs["name"].to_string().replace("\"", "");
        if name.contains(&word) {
            asset_name = name;
        }
    }
    (release["tag_name"].to_string().replace("\"", ""), asset_name)
}
