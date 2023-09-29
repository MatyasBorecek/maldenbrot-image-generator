mod structs;

use structs::{MandelbrotConfig::MandelbrotConfig, Point::Point};
use image::{Rgb, ImageBuffer};

const IMGX: u32 = 800;
const IMGY: u32 = 800;
const SCALEX: f64 = 4.0 / IMGX as f64;
const SCALEY: f64 = 4.0 / IMGY as f64;
const MAX_ITERATION: u32 = 40;

fn main() {
    let config = MandelbrotConfig::new(IMGX, IMGY, SCALEX, SCALEY);
    let imgbuf = generate_mandelbrot_image(&config);
    imgbuf.save("fractal.png").expect("Failed to save image");
}

fn generate_mandelbrot_image(config: &MandelbrotConfig) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut imgbuf = ImageBuffer::new(config.width, config.height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let point = Point {
            x: x as f64 * config.scale_x - 2.0,
            y: y as f64 * config.scale_y - 2.0,
        };
        let color = calculate_mandelbrot_color(&point);
        *pixel = Rgb(color);
    }

    imgbuf
}

fn calculate_mandelbrot_color(point: &Point) -> [u8; 3] {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut iteration = 0;

    while x.powi(2) + y.powi(2) < 4.0 && iteration < MAX_ITERATION {
        let x_temp = x.powi(2) - y.powi(2) + point.x;
        y = 2.0 * x * y + point.y;
        x = x_temp;
        iteration += 1;
    }

    match iteration {
        MAX_ITERATION => [0, 0, 0],
        _ => {
            let value = ((iteration as f64 / MAX_ITERATION as f64) * 255.0) as u8;
            [value, value, value]
        }
    }
}
