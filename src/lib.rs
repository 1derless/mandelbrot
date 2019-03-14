mod configuration;
mod mandelbrot;
mod region;

pub use configuration::Configuration;
pub use mandelbrot::{iterate_mandelbrot, MandelResult};
use region::Region;

use image::imageops::replace;
use image::RgbImage;
use num_complex::Complex;
use palette::{Gradient, LinSrgb};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn mandelbrot_loop(
    regions: Arc<Mutex<Vec<Region>>>,
    config: Configuration,
    grad_func: &(Fn(MandelResult) -> image::Rgb<u8>),
    tx: Sender<(
        Region,
        image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
    )>,
) {
    // Loops until there are no more regions in the shared Vector.
    loop {
        let region = match regions.lock() {
            Ok(mut regions) => match regions.pop() {
                Some(region) => region,
                None => break,
            },
            Err(_) => continue,
        };

        let mut img = RgbImage::new(region.w, region.h);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let c = Complex::new(
                (region.x + x) as f64 / config.output_w as f64 * (config.x_max - config.x_min)
                    + config.x_min,
                (region.y + y) as f64 / config.output_h as f64 * (config.y_max - config.y_min)
                    + config.y_min,
            );

            *pixel = grad_func(iterate_mandelbrot(c));
        }
        tx.send((region, img)).unwrap();
    }
}

pub fn run(config: Configuration) {
    // The gradient isn't constant because its type is complicated.
    let grad = Gradient::new(vec![
        LinSrgb::new(1.0, 0.0, 0.0),
        LinSrgb::new(0.1, 0.1, 0.1),
        LinSrgb::new(0.0, 1.0, 0.0),
        LinSrgb::new(0.1, 0.1, 0.1),
        LinSrgb::new(0.0, 0.0, 1.0),
        LinSrgb::new(0.1, 0.1, 0.1),
        LinSrgb::new(1.0, 0.0, 0.0),
    ]);

    // Creates tiny views of the mandelbrot set fractal and puts them in a thread-safe vector.
    let output_region = Region::new(config.output_w, config.output_h);
    let regions = Arc::new(Mutex::new(
        output_region.clone().split(Region::new(256, 256)),
    ));
    let no_regions = regions.lock().unwrap().len();

    // Creates the shared objects.
    let (tx, rx) = channel();
    let mut handles = vec![];

    // Initialises and starts the threads.
    for _ in 0..num_cpus::get() {
        let tx = tx.clone();
        let grad = grad.clone();
        let regions = Arc::clone(&regions);
        let config = config.clone();

        let colour = move |c| match c {
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

        let handle = thread::spawn(move || mandelbrot_loop(regions, config, &colour, tx));
        handles.push(handle);
    }

    // Stitches output regions back together.
    let mut master_image = RgbImage::new(output_region.w, output_region.h);
    for _ in 0..no_regions {
        let (region, img) = rx.recv().unwrap();
        replace(&mut master_image, &img, region.x, region.y);
    }

    // Rejoins with worker threads.
    for handle in handles {
        handle.join().unwrap();
    }

    // Saves resultant image to disk.
    master_image.save(config.file).unwrap();
}
