use std::path::PathBuf;

use clap::{App, Arg};

#[derive(Clone, Debug)]
pub struct CLIArgs {
    pub posts_path: PathBuf,
    pub templates_path: PathBuf,
    pub output_path: PathBuf,
    pub static_path: PathBuf,
    pub cname_path: PathBuf,
    pub favicon_path: PathBuf,
    pub num_cards: u64,
}

pub fn parse_args() -> CLIArgs {
    let matches = App::new("build-website")
        .version("0.1")
        .author("Quang Duong <duongquang1@gmail.com>")
        .about("Build personal website")
        .arg(Arg::with_name("posts")
            .long("posts")
            .takes_value(true)
             .value_name("PATH")
            .default_value("posts")
            .help("Path to root directory of posts"))
        .arg(Arg::with_name("templates")
            .long("templates")
            .takes_value(true)
             .value_name("PATH")
            .default_value("templates")
            .help("Path to root directory of templates"))
        .arg(Arg::with_name("static")
            .long("static")
            .takes_value(true)
             .value_name("PATH")
            .default_value("static")
            .help("Path to directory to static files"))
        .arg(Arg::with_name("output")
            .long("output")
            .takes_value(true)
             .value_name("PATH")
            .default_value("docs")
            .help("Path to root directory to output"))
        .arg(Arg::with_name("cname-path")
             .long("cname-path")
             .takes_value(true)
             .value_name("PATH")
             .default_value("CNAME")
             .help("Path to CNAME file"))
        .arg(Arg::with_name("favicon-path")
            .long("favicon-path")
            .takes_value(true)
             .value_name("PATH")
            .default_value("favicon")
            .help("Path to root directory of favicon data"))
        .arg(Arg::with_name("num-cards")
            .long("num-cards")
            .takes_value(true)
             .value_name("NUM")
            .default_value("5")
            .help("Number of cards on a page"))
        .get_matches();

    CLIArgs {
        posts_path: PathBuf::from(matches.value_of("posts").unwrap()),
        templates_path: PathBuf::from(matches.value_of("templates").unwrap()),
        output_path: PathBuf::from(matches.value_of("output").unwrap()),
        static_path: PathBuf::from(matches.value_of("static").unwrap()),
        cname_path: PathBuf::from(matches.value_of("cname-path").unwrap()),
        favicon_path: PathBuf::from(matches.value_of("favicon-path").unwrap()),
        num_cards: matches.value_of("num-cards").unwrap().parse::<u64>().unwrap(),
    }
}
