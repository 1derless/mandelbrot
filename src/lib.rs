mod configuration;
mod mandelbrot;

pub use configuration::Configuration;
pub use mandelbrot::{iterate_mandelbrot, MandelResult};

use image::RgbImage;
use num_complex::Complex;
use palette::{Gradient, LinSrgb};
use rayon::prelude::*;

pub fn run(config: Configuration) {
    // The gradient isn't constant because its type is complicated.
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
    ]);

    //todo: separate below into distinct functions.

    // Iterates over all coordinates, in parallel.
    let mut results: Vec<MandelResult> = (0..config.output_h)
        .into_par_iter()
        .flat_map(|y| {
            (0..config.output_w)
                .into_par_iter()
                .rev()
                .map(move |x| (x, y))
        })
        .map(|(x, y)| {
            let c = Complex::new(
                x as f64 / config.output_w as f64 * (config.x_max - config.x_min) + config.x_min,
                y as f64 / config.output_h as f64 * (config.y_max - config.y_min) + config.y_min,
            );

            iterate_mandelbrot(c)
        })
        .collect();

    let mut master_image = RgbImage::new(config.output_w, config.output_h);

    println!("Done finding escape velocity, now colouring by numbers.");

    // Not really worth parallelising seeing as it is so quick.
    for pixel in master_image.pixels_mut() {
        //todo: optimisation to stop using pop here below.
        if let Some(result) = results.pop() {
            let colour = match result {
                MandelResult::Outside(i) => {
                    let components = grad.get((i / 25.0) % 1.0);
                    image::Rgb([
                        (components.red * 255.0) as u8,
                        (components.green * 255.0) as u8,
                        (components.blue * 255.0) as u8,
                    ])
                }
                MandelResult::Inside => image::Rgb([0u8, 0u8, 0u8]),
            };
            *pixel = colour;
        }
    }

    // Saves resultant image to disk.
    master_image.save(config.file).unwrap();
}
