mod configuration;
mod mandelbrot;

pub use configuration::Configuration;
pub use mandelbrot::{iterate_mandelbrot, MandelResult};

use image::RgbImage;
use num_complex::Complex;
use palette::{Gradient, LinSrgb};
use rayon::prelude::*;

pub fn run(config: Configuration) {
    let grad = Gradient::new(vec![
        /* LinSrgb::new(1.0, 0.0, 0.0),
        LinSrgb::new(0.1, 0.1, 0.1),
        LinSrgb::new(0.0, 1.0, 0.0),
        LinSrgb::new(0.1, 0.1, 0.1),
        LinSrgb::new(0.0, 0.0, 1.0),
        LinSrgb::new(0.1, 0.1, 0.1),
        LinSrgb::new(1.0, 0.0, 0.0),*/
        LinSrgb::new(0.0, 0.5, 1.0),
        LinSrgb::new(1.0, 1.0, 1.0),
        LinSrgb::new(0.5, 1.0, 0.0),
        LinSrgb::new(1.0, 1.0, 1.0),
        LinSrgb::new(1.0, 0.0, 0.5),
        LinSrgb::new(1.0, 1.0, 1.0),
        LinSrgb::new(0.0, 0.5, 1.0),
    ]);

    // Iterates over all coordinates, in parallel.
    let results = (0..config.output_h)
        .into_par_iter()
        .flat_map(|y| (0..config.output_w).into_par_iter().map(move |x| (x, y)))
        // Computes whether the current number is within the set / its escape speed.
        .map(|(x, y)| {
            let c = Complex::new(
                x as f64 / config.output_w as f64 * (config.x_max - config.x_min) + config.x_min,
                y as f64 / config.output_h as f64 * (config.y_max - config.y_min) + config.y_min,
            );

            iterate_mandelbrot(c)
        })
        // Translates escape speed to colour.
        .map(|result| match result {
            MandelResult::Outside(i) => {
                let components = grad.get((i / 25.0) % 1.0);
                image::Rgb([
                    (components.red * 255.0) as u8,
                    (components.green * 255.0) as u8,
                    (components.blue * 255.0) as u8,
                ])
            }
            MandelResult::Inside => image::Rgb([0u8, 0u8, 0u8]),
        })
        .collect::<Vec<image::Rgb<u8>>>();


    // Creates an image made from the results.
    let mut master_image = RgbImage::new(config.output_w, config.output_h);
    for (pixel, colour) in master_image.pixels_mut().zip(results) {
        *pixel = colour;
    }

    // Saves resultant image to disk.
    master_image.save(config.file).unwrap();
}
