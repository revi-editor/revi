// TODO: Move this file into main.
use crate::AUTHOR;
use clap::{crate_description, crate_name, crate_version, values_t, App, Arg};

pub fn args() -> Vec<String> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(AUTHOR)
        .about(crate_description!())
        .arg(Arg::with_name("files").multiple(true))
        .after_help("Pass in any number of files to ReVi to be placed in the Buffer list.")
        .get_matches();

    values_t!(matches, "files", String).unwrap_or_default()
}
