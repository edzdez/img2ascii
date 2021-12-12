use anyhow::{Context, Result};
use image::io::Reader;
use image::{DynamicImage, ImageResult, Rgb};
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::convertor::*;
use crate::options::Options;

mod convertor;
mod options;

fn read_image(filepath: &PathBuf) -> ImageResult<DynamicImage> {
    Ok(Reader::open(filepath)?.with_guessed_format()?.decode()?)
}

fn write_image(
    ascii_matrix: &Vec<String>,
    color_matrix: &Vec<Vec<Rgb<u8>>>,
    foreground: bool,
    background: bool,
) {
    for (y, line) in ascii_matrix.iter().enumerate() {
        for (x, pixel) in line.chars().enumerate() {
            let color = color_matrix[y][x].0;
            if foreground {
                print!("\x1b[38;2;{};{};{}m", color[0], color[1], color[2]);
            }
            if background {
                print!("\x1b[48;2;{};{};{}m", color[0], color[1], color[2]);
            }
            print!("{}", pixel);
            if foreground || background {
                print!("\x1b[0m");
            }
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::from_args();

    let image = read_image(&opts.input_file)
        .with_context(|| format!("could not open image {:?}", &opts.input_file))?;

    write_image(
        &image_to_ascii_matrix(&opts, &image),
        &image_to_color_matrix(&opts, &image),
        opts.color_foreground,
        opts.color_background,
    );

    Ok(())
}
