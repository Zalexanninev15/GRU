use isahc::ReadResponseExt;
use serde_json::Value;

// Main function of parser
pub fn parse_data(repo: &str, search_words: &str, is_pre: &bool) -> (String, String) {
    let json = get_text(&repo, &is_pre);
    let release_data = parse_text(&json, &search_words);
    release_data
}

// Parse json with release data
fn parse_text(json: &str, word: &str) -> (String, String) {
    let release: Value = serde_json::from_str(json).expect("GitHub API: Error parsing json");
    let mut slob = String::from("app.zip");
    for rs in release["assets"].as_array().unwrap() {
        let name = rs["name"].to_string().replace("\"", "");
        if name.contains(&word) {
            slob = name;
        }
    }
    (release["tag_name"].to_string().replace("\"", ""), slob)
}

// Getting release information in json format
fn get_text(repo: &str, is_pre: &bool) -> String {
	let mut release_json : String = String::new();
	if *is_pre {
		release_json = isahc::get(String::from(format!(
			"https://api.github.com/repos/{}/releases?per_page=1&page=1",
			repo
		)))
		.expect("GitHub API: Error 404")
		.text()
		.expect("GitHub API: Json lost");
	}
	else {
		release_json = isahc::get(String::from(format!(
			"https://api.github.com/repos/{}/releases/latest",
			repo
		)))
		.expect("GitHub API: Error 404")
		.text()
		.expect("GitHub API: Json lost");
	}
    release_json
}
