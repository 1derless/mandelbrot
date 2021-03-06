use argparse::{ArgumentParser, Store, StoreTrue};

#[derive(Clone)]
pub struct Configuration {
    pub output_w: u32,
    pub output_h: u32,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub file: String,
    pub silent: bool,
}

impl Configuration {
    pub fn from_args() -> Configuration {
        let default = Configuration {
            output_w: 512,
            output_h: 512,
            x_min: -2.0, // -2.0
            x_max: 0.6,  //  0.6
            y_min: -1.3, // -1.3
            y_max: 1.3,  //  1.3
            file: String::from("output.png"),
            silent: false,
        };

        let mut res_string = String::from("512x512");
        let mut x_min = String::from("-2.0");
        let mut x_max = String::from("0.6");
        let mut y_min = String::from("-1.3");
        let mut y_max = String::from("1.3");
        let mut file = String::from("output.png");
        let mut silent = false;

        {
            let mut parser = ArgumentParser::new();
            parser.set_description(
                "A mandelbrot set plotter. \
                 Any unspecified option will be left as default.",
            );

            parser.refer(&mut res_string).add_option(
                &["-r", "--resolution"],
                Store,
                "Specifies custom output resolution, \
                 in the format that fits the format [width]x[height].",
            );
            parser.refer(&mut x_min).add_option(
                &["-x", "--min_x"],
                Store,
                "Specifies custom minimum real value.",
            );
            parser.refer(&mut x_max).add_option(
                &["-X", "--max_x"],
                Store,
                "Specifies custom maximum real value.",
            );
            parser.refer(&mut y_min).add_option(
                &["-y", "--min_y"],
                Store,
                "Specifies custom minimum imaginary value.",
            );
            parser.refer(&mut y_max).add_option(
                &["-Y", "--max_y"],
                Store,
                "Specifies custom maximum imaginary value.",
            );
            parser.refer(&mut file).add_option(
                &["-o", "--output"],
                Store,
                "Specifies custom output file location. \
                 A valid format must also be specified though the file extension.",
            );
            parser.refer(&mut silent).add_option(
                &["-s", "--silent"],
                StoreTrue,
                "If absent, prints the time taken to stdout.",
            );

            parser.parse_args_or_exit();
        }

        let res_string = res_string.to_string();
        let mut dimentions = res_string.split('x');

        Configuration {
            output_w: dimentions
                .next()
                .unwrap()
                .parse()
                .unwrap_or_else(|_| default.output_w),
            output_h: dimentions
                .next()
                .unwrap()
                .parse()
                .unwrap_or_else(|_| default.output_h),
            x_min: x_min.parse().unwrap_or(default.x_min),
            x_max: x_max.parse().unwrap_or(default.x_max),
            y_min: y_min.parse().unwrap_or(default.y_min),
            y_max: y_max.parse().unwrap_or(default.y_max),
            file,
            silent,
        }
    }
}
