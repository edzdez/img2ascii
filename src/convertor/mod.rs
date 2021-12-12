use crate::Options;
use image::{DynamicImage, Pixel, Rgb, RgbImage};

pub fn get_avg_luminance(image: &RgbImage, x: u32, y: u32, scale: u32) -> u32 {
    let mut luminance: u32 = 0;
    for new_y in y..y + 2 * scale {
        for new_x in x..x + scale {
            let pixel = image.get_pixel(new_x, new_y);
            luminance += pixel.to_luma().0[0] as u32;
        }
    }

    luminance / (scale * 2 * scale)
}

pub fn get_avg_color(image: &RgbImage, x: u32, y: u32, scale: u32) -> Rgb<u8> {
    let (mut r, mut g, mut b) = (0, 0, 0);
    for new_y in y..y + 2 * scale {
        for new_x in x..x + scale {
            let pixel = image.get_pixel(new_x, new_y);
            r += pixel.to_rgb().0[0] as u32;
            g += pixel.to_rgb().0[1] as u32;
            b += pixel.to_rgb().0[2] as u32;
        }
    }

    Rgb([
        (r / (scale * 2 * scale)) as u8,
        (g / (scale * 2 * scale)) as u8,
        (b / (scale * 2 * scale)) as u8,
    ])
}

pub fn pixel_to_char(opts: &Options, image: &RgbImage, x: u32, y: u32) -> char {
    let ramp = &opts.ramp;
    let ramp_len = ramp.len() as u32;

    let luma = get_avg_luminance(image, x, y, opts.scale);
    ramp.chars()
        .nth(f64::ceil(((ramp_len - 1) * luma) as f64 / 255 as f64) as usize)
        .unwrap()
}

pub fn image_to_ascii_matrix(opts: &Options, image: &DynamicImage) -> Vec<String> {
    let image = image.to_rgb8();

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

pub fn image_to_color_matrix(opts: &Options, image: &DynamicImage) -> Vec<Vec<Rgb<u8>>> {
    let image = image.to_rgb8();

    let mut matrix = Vec::new();
    for y in (0..image.height() - 2 * opts.scale).step_by(2 * opts.scale as usize) {
        let mut line = Vec::new();
        for x in (0..image.width() - opts.scale).step_by(opts.scale as usize) {
            line.push(get_avg_color(&image, x, y, opts.scale as u32));
        }
        matrix.push(line);
    }

    matrix
}
