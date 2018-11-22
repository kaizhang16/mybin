use clap::{clap_app, crate_authors, crate_version};
use dirs::home_dir;
use mybin::md2pdf;
use std::path::Path;
use std::process::exit;

fn main() {
    let app = clap_app!(mybin =>
        (about: "My binaries")
        (author: crate_authors!())
        (version: crate_version!())
        (@subcommand md2pdf =>
            (about: "Convert markdown to pdf")
            (@arg INPUT: +required "the input file")
        )
    )
    .get_matches();

    match app.subcommand() {
        ("md2pdf", Some(subcommand)) => {
            let input_file = subcommand.value_of("INPUT").unwrap();
            let exit_code = md2pdf(Path::new(input_file), home_dir().unwrap().as_path());
            exit(exit_code)
        }
        _ => {}
    }
}
