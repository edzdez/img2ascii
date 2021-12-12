use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}