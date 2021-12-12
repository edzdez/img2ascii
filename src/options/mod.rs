use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    /// Enable/disable color
    #[structopt(short = "c", long = "color")]
    pub color_output: bool,

    /// Set scale for the resulting image
    #[structopt(short = "s", long = "scale", default_value = "2")]
    pub scale: u32,

    /// Set character ramp
    #[structopt(short = "r", long = "ramp", default_value = r##"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'."##)]
    pub ramp: String,

    /// Input file
    #[structopt(parse(from_os_str))]
    pub input_file: PathBuf,
}
