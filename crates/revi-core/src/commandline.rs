use crate::AUTHOR;
use clap::{crate_description, crate_name, crate_version, App, Arg};
use ropey::Rope;
use std::fs::{metadata, OpenOptions};
use std::io::BufReader;

pub fn argparser() -> Option<String> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(AUTHOR)
        .about(crate_description!())
        .arg(Arg::with_name("in_file").index(1))
        .after_help(
            "Longer explanation to appear after the options when \
             displaying the help information from --help or -h",
        )
        .get_matches();

    matches.value_of("in_file").map(|v| v.to_string())
}

// wrapper around Rope for a drity flag.
pub fn from_path(path: Option<String>) -> (Rope, Option<String>) {
    let text = path
        .as_ref()
        .filter(|path| metadata(&path).is_ok())
        .map_or_else(Rope::new, |path| {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .expect("Problem opening the file");

            Rope::from_reader(BufReader::new(file)).unwrap()
        });

    (text, path)
}
