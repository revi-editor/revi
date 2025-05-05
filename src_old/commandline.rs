use super::AUTHOR;
use clap::{crate_description, crate_name, crate_version, Parser};

#[derive(Debug, Parser)]
#[command(
    about = "A fictional versioning CLI",
    name = crate_name!(),
    author = AUTHOR,
    long_about = Some(crate_description!()),
    color = clap::ColorChoice::Always,
    version=crate_version!(),
)]
pub struct Cli {
    pub files: Vec<String>,
}
