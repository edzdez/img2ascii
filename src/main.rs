use anyhow::{Context, Result};
use image::io::Reader;
use image::{DynamicImage, ImageResult, Pixel, RgbImage};
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::options::Options;

mod options;

fn read_image(filepath: &PathBuf) -> ImageResult<DynamicImage> {
    Ok(Reader::open(filepath)?.with_guessed_format()?.decode()?)
}

fn get_avg_luminance(image: &RgbImage, x: u32, y: u32, scale: u32) -> u32 {
    let mut luminance: u32 = 0;
    for new_y in y..y + 2 * scale {
        for new_x in x..x + scale {
            let pixel = image.get_pixel(new_x, new_y);
            luminance += pixel.to_luma().0[0] as u32;
        }
    }

    luminance / (scale * 2 * scale)
}

fn pixel_to_char(opts: &Options, image: &RgbImage, x: u32, y: u32) -> char {
    let ramp = &opts.ramp;
    let ramp_len = ramp.len() as u32;

    let luma = get_avg_luminance(image, x, y, opts.scale);
    ramp.chars().nth(f64::ceil(((ramp_len - 1) * luma) as f64 / 255 as f64) as usize).unwrap()
}

fn image_to_ascii_matrix(opts: &Options, image: &DynamicImage) -> Vec<String> {
    let image = match opts.color_output {
        true => image.to_rgb8(),
        false => image.grayscale().to_rgb8(),
    };

    let mut matrix = Vec::new();
    for y in (0..image.height() - 2 * opts.scale).step_by(2 * opts.scale as usize) {
        let mut line = String::new();
        for x in (0..image.width() - opts.scale).step_by(opts.scale as usize) {
            line.push(pixel_to_char(opts, &image, x, y))
        }
        matrix.push(line);
    }

    matrix
}

fn write_image(ascii_matrix: &Vec<String>) {
    for line in ascii_matrix {
        for pixel in line.chars() {
            print!("{}", pixel);
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::from_args();

    let image = read_image(&opts.input_file)
        .with_context(|| format!("could not open image {:?}", &opts.input_file))?;

    write_image(&image_to_ascii_matrix(&opts, &image));

    Ok(())
}
