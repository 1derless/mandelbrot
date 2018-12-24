extern crate num_complex;

use self::num_complex::Complex;

const MAX_ITERATIONS: u8 = 255;
const MAX_MAGNITUDE: f64 = 1024.0;

pub enum MandelResult {
    Inside,
    Outside(f64),
}

pub fn iterate_mandelbrot(c: Complex<f64>) -> MandelResult {
    let mut x = Complex::new(0.0, 0.0);

    for i in 0..MAX_ITERATIONS {
        x = x.powf(2.0) + c;

        if x.norm_sqr() > MAX_MAGNITUDE.powi(2) {
            let i = i as f64 + 1.0 - (x.norm_sqr() / 2.0).ln().log(2.0);
            //let i = i as f64 + 1.0 - (x.log(2.0).log(2.0) / (2.0f64).log(2.0));
            return MandelResult::Outside(i);
        }
    }

    MandelResult::Inside
}
