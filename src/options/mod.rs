use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    /// Enable/disable color for foreground
    #[structopt(short = "f", long = "fcolor")]
    pub color_foreground: bool,

    /// Enable/disable color for background
    #[structopt(short = "b", long = "bcolor")]
    pub color_background: bool,

    /// Set scale for the resulting image
    #[structopt(short = "s", long = "scale", default_value = "5")]
    pub scale: u32,

    /// Set character ramp
    #[structopt(short = "r", long = "ramp", default_value = r##"@#8&|o:*_. "##)]
    pub ramp: String,

    /// Input file
    #[structopt(parse(from_os_str))]
    pub input_file: PathBuf,
}
