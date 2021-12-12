use std::error::Error;
use std::path::PathBuf;
use image::{DynamicImage, ImageResult};
use image::io::Reader;
use structopt::StructOpt;

use crate::options::Options;

mod options;

fn read_image(filepath: &PathBuf) -> ImageResult<DynamicImage> {
    Ok(Reader::open(filepath)?
        .with_guessed_format()?
        .decode()?)
}

fn main() -> Result<(), Box<dyn Error>>{
    let opts = Options::from_args();

    let image = read_image(&opts.input)?;

    Ok(())
}
