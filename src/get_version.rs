use std::fs::File;
use std::io::Read;
use std::path::Path;

// Checker for the versions
pub fn is_new_version(new_version: &str, application_path: &str) -> i32 {
    if Path::new("app.version").exists() && Path::new(&application_path).exists() {
        let real_version = get_version_from_file();
        println!("\nCurrent version of app: {}", real_version);
        if new_version == real_version {
            return 0;
        } else {
            return 1;
        }
    } else {
        println!("\nCurrent version of app: <not detected>");
        return 1;
    }
}

// Get version from file as GitHub release tag
fn get_version_from_file() -> String {
    let mut file_read =
        File::open("app.version").expect("Error opening file with version information!");
    let mut file_data = String::new();
    file_read
        .read_to_string(&mut file_data)
        .expect("Error reading file with version information!");
    file_data
}
