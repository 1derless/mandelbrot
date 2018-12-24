extern crate image_mandelbrot;

use image_mandelbrot::{run, Configuration};

fn main() {
    // Parses the command line arguments.
    let config = Configuration::from_args();

    run(config);
}
