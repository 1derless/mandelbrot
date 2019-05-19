//todo: reduce the size of `MandelResult` by using NaN or inf to represent the variant `Inside`.

use num_complex::Complex;

const POWER: f64 = 2.0;
const MAX_ITERATIONS: u8 = 255;
// Apparently this only needs to be 2.0, thanks Ben Sparks from Numberphile.
#[cfg(not(feature = "smoothing"))]
const MAX_MAGNITUDE: f64 = 2.0;
// But a number so low does mess with the smoothing algorithm.
#[cfg(feature = "smoothing")]
const MAX_MAGNITUDE: f64 = 1024.0;

pub enum MandelResult {
    Inside,
    Outside(f64),
}

pub fn iterate_mandelbrot(c: Complex<f64>) -> MandelResult {
    let mut x = Complex::new(0.0, 0.0);

    for i in 0..MAX_ITERATIONS {
        x = x.powf(POWER) + c;

        if x.norm_sqr() > MAX_MAGNITUDE.powi(2) {
            // Removing the smoothing does not seem to save any significant time,
            // but it does look nicer with this gradient, no?
            #[cfg(feature = "smoothing")]
            let i = i as f64 - (x.norm_sqr() / 2.0).log(MAX_MAGNITUDE).log(POWER);

            return MandelResult::Outside(i as f64);
        }
    }

    MandelResult::Inside
}
