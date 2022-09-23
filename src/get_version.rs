use std::fs::File;
use std::io::Read;
use std::path::Path;
// use pelite::FileMap;

// Checker for the versions
pub fn is_new_version(new_version: &str, application_path: &str) -> i32 {
    let mut real_version = "0.0.0.0".to_string();
    if Path::new("app.version").exists() && Path::new(&application_path).exists() {
        real_version = get_version_from_file();
        println!("\nCurrent version of app: {}", real_version);
        if new_version == real_version {
            return 0
        }
        else {
            return  1
        }
    }
    else {
        println!("\nCurrent version of app: <not detected>");
        return  1
    }
    // let mut current_version = parse_pe_version(application_path);
    // println!("\nCurrent version of app: {}", current_version);
    // current_version = current_version
    //     .to_lowercase()
    //     .replace(",", ".")
    //     .replace(" ", "")
    //     .replace(".", "")
    //     .replace("build", "")
    //     .replace("v", "")
    //     .replace("-", "");
    // let new_version_c = &new_version
    //     .to_lowercase()
    //     .replace(".", "")
    //     .replace("v", "")
    //     .replace("build", "")
    //     .replace("ersion", "")
    //     .replace(" ", "")
    //     .replace("_", "")
    //     .replace("-", "");
    // if current_version.contains(new_version_c) {
    //     0
    // } else {
    //     if current_version == "" {
    //         -1
    //     } else {
    //         1
    //     }
    // }
}

// Get version from file as GitHub release tag
fn get_version_from_file() -> String {
    let mut file_read = File::open("app.version").expect("Error opening file with version information!");
    let mut file_data = String::new();
    file_read.read_to_string(&mut file_data).expect("Error reading file with version information!");
    file_data
}

// Function for getting the file version from PE file
// fn parse_pe_version(file_path: &str) -> String {
//     match FileMap::open(file_path) {
//         Err(_) => "".to_string(),
//         Ok(file_map) => {
//             let result = match pelite::PeFile::from_bytes(&file_map) {
//                 Err(e) => Err(e),
//                 Ok(f) => f.resources(),
//             };
//             match result {
//                 Err(_) => "".to_string(),
//                 Ok(resources) => match resources.version_info() {
//                     Err(_) => "".to_string(),
//                     Ok(version_info) => match version_info.fixed() {
//                         None => "".to_string(),
//                         Some(fixed_info) => {
//                             let version = fixed_info.dwFileVersion.to_string();
//                             if version.len() > 0 {
//                                 version
//                             } else {
//                                 fixed_info.dwProductVersion.to_string()
//                             }
//                         }
//                     },
//                 },
//             }
//         }
//     }
// }
