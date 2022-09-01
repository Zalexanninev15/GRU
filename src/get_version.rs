use pelite::PeFile;

// Checker for the versions
pub fn is_new_version(new_version: &str, application_path: &str) -> i32 {
    let file_map =
        pelite::FileMap::open(application_path).expect("Cannot open the file specified!");
    let image = pelite::PeFile::from_bytes(file_map.as_ref()).expect("File is not a PE image!");
    let current_version = get_file_version(image).replace(", ", "");
    if new_version.contains(&current_version) {
        1
    } else {
        if current_version == "" {
            -1
        } else {
            0
        }
    }
}

// Function for getting the file version from PE file
fn get_file_version(bin: PeFile<'_>) -> String {
    let resources = bin.resources().expect("Resources not found!");
    let lang: Option<u16> = None;
    let version_info = match lang {
        Some(lang) => resources
            .find_resource_ex(&[pelite::resources::Name::VERSION, 1.into(), lang.into()])
            .and_then(|bytes| {
                Ok(pelite::resources::version_info::VersionInfo::try_from(
                    bytes,
                )?)
            }),
        None => resources.version_info(),
    }
    .expect("Version info not found!");
    let lang = version_info.translation()[0];
    let file_version = String::from(format!(
        "{}",
        version_info.value(lang, "ProductVersion").unwrap()
    ));
    file_version
}
