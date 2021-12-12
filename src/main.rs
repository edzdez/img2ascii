use anyhow::{Context, Result};
use image::io::Reader;
use image::{DynamicImage, ImageResult};
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::options::Options;

mod options;

fn read_image(filepath: &PathBuf) -> ImageResult<DynamicImage> {
    Ok(Reader::open(filepath)?.with_guessed_format()?.decode()?)
}

fn main() -> Result<()> {
    let opts = Options::from_args();

    let image = read_image(&opts.input)
        .with_context(|| format!("could not open image {:?}", &opts.input))?;

    Ok(())
}
