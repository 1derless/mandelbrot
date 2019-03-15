/*
#![feature(test)]
extern crate test;
*/

use image_mandelbrot::{run, Configuration};
use separator::Separatable;

fn main() {
    // Parses the command line arguments.
    let config = Configuration::from_args();

    if config.silent {
        run(config);
    } else {
        let duration = time::Duration::span(|| run(config));

        println!(
            "Done in {} ms.",
            duration.num_milliseconds().separated_string()
        );
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use image_mandelbrot::iterate_mandelbrot;
    use num_complex::Complex;

    #[bench]
    fn bench_single(b: &mut Bencher) {
        b.iter(|| iterate_mandelbrot(Complex { re: 1.0, im: 1.0 }));
    }
}
*/
