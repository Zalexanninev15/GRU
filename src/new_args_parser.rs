use clap::{App, Arg};

pub fn cli_parser() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    let matches = App::new("GitHub Release Updater")
        .version(VERSION)
        .author("Zalexanninev15 <blue.shark@disroot.org>")
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("repository")
                .short("r")
                .long("repo")
                .takes_value(true)
                .help("Application repository on GitHub: {{user/repository}}"),
        )
        .arg(
            Arg::with_name("search_this")
                .short("s")
                .long("search")
                .takes_value(true)
                .help("Set the part of name of asset in GitHub release for download (several parts of the name can be used, as long as they are separated by a space and enclosed in quotation marks, for example: \"win amd64 portable\")"),
        )
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);
}
