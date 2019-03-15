use num_complex::Complex;

const MAX_ITERATIONS: u8 = 255;
const MAX_MAGNITUDE: f64 = 1024.0;

pub enum MandelResult {
    Inside,
    Outside(f64),
}

pub fn iterate_mandelbrot(c: Complex<f64>) -> MandelResult {
    let mut x = Complex::new(0.0, 0.0);

    for i in 0..MAX_ITERATIONS {
        x = x.powf(3.0 + 8.0/9.0) + c;

        if x.norm_sqr() > MAX_MAGNITUDE.powi(2) {
            // Removing the smoothing does not seem to save any time,
            // but it does look nicer with this gradient, no?
            //let i = i as f64 + 1.0 - x.norm().ln().log(MAX_MAGNITUDE);

            //todo: move this constant offset onto the gradient.
            return MandelResult::Outside(i as f64 - 2.75);
        }
    }

    MandelResult::Inside
}
