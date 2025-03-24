use std::fs::File;
use std::io::Read;
use std::path::Path;

// Checker for the versions
pub fn is_new_version(new_version: &str, application_path: &str) -> i32 {
    if Path::new("app.version").exists() && Path::new(&application_path).exists() {
        let real_version = get_version_from_file();
        println!("\nТекущая версия приложения: {}", real_version);
        if new_version == real_version {
            return 0;
        } else {
            return 1;
        }
    } else {
        println!("\nТекущая версия приложения: <нет информации>");
        return 1;
    }
}

// Get version from file as GitHub release tag
fn get_version_from_file() -> String {
    let mut file_read =
        File::open("app.version").expect("Ошибка при открытии файла с информацией о версии!");
    let mut file_data = String::new();
    file_read
        .read_to_string(&mut file_data)
        .expect("Ошибка при чтении файла с информацией о версии!");
    file_data
}
