extern crate image_mandelbrot;
extern crate time;
extern crate separator;

use image_mandelbrot::{run, Configuration};
use separator::Separatable;

fn main() {
    // Parses the command line arguments.
    let config = Configuration::from_args();

    if config.silent {
        run(config);
    } else {
        let duration = time::Duration::span(|| run(config));

        println!("Done in {} ms.", duration.num_milliseconds().separated_string());
    }
}
