use pelite::FileMap;

// Checker for the versions
pub fn is_new_version(new_version: &str, application_path: &str) -> i32 {
    let mut current_version = parse_pe_version(application_path);
    println!("\nCurrent version of app: {}", current_version);
    current_version = current_version
        .replace(",", ".")
        .replace(" ", "")
        .replace(".", "");
    let new_version_c = &new_version
        .replace(".", "")
        .replace("v", "")
        .replace("build", "");
    if current_version.contains(new_version_c) {
        0
    } else {
        if current_version == "" {
            -1
        } else {
            1
        }
    }
}

// Function for getting the file version from PE file
fn parse_pe_version(file_path: &str) -> String {
    match FileMap::open(file_path) {
        Err(_) => "".to_string(),
        Ok(file_map) => {
            let result = match pelite::PeFile::from_bytes(&file_map) {
                Err(e) => Err(e),
                Ok(f) => f.resources(),
            };
            match result {
                Err(_) => "".to_string(),
                Ok(resources) => match resources.version_info() {
                    Err(_) => "".to_string(),
                    Ok(version_info) => match version_info.fixed() {
                        None => "".to_string(),
                        Some(fixed_info) => {
                            let version = fixed_info.dwFileVersion.to_string();
                            if version.len() > 0 {
                                version
                            } else {
                                fixed_info.dwProductVersion.to_string()
                            }
                        }
                    },
                },
            }
        }
    }
}
