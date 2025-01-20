use image::{RgbImage, Rgb};
use std::time::Instant;


const  MAX_ITERATIONS: u16 = 1000;

// const HEIGHT: u32 = 4320;
// const WIDTH: u32 = 7680;

const HEIGHT: u32 = 500;
const WIDTH: u32 = 500;

const MIN_REAL: f64 = -2.0;
const MAX_REAL: f64 = 1.0;
const MIN_IMAG: f64 = -1.5;
const MAX_IMAG: f64 = 1.5;


struct ComplexNumber {
    a: f64,
    b: f64
}

impl ComplexNumber {
    fn new(a: f64, b: f64) -> Self {
        ComplexNumber{ a, b }
    }

    fn add(&mut self, c: &ComplexNumber) {
        self.a += c.a;
        self.b += c.b;
    }

    fn magnitude(&self) -> f64 {
        return (self.a.powi(2) + self.b.powi(2)).sqrt().abs()
    }

    fn squared(&mut self) {
        let temp_a: f64 = self.a.powi(2) - self.b.powi(2);
        let temp_b: f64 = 2.0 * self.a * self.b;

        self.a = temp_a;
        self.b = temp_b;
    }
}


fn main() {

    let start_time = Instant::now();

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {

            let  mut iteration: u16 = 0;

            let mut z = ComplexNumber::new(0.0, 0.0);

            let cx = MIN_REAL + (x as f64 / WIDTH as f64) * (MAX_REAL - MIN_REAL);
            let cy = MIN_IMAG + (y as f64 / HEIGHT as f64) * (MAX_IMAG - MIN_IMAG);


            let c = ComplexNumber::new(cx as f64,cy as f64);            

            while (iteration < MAX_ITERATIONS) {
                iteration += 1;

                z.squared();
                z.add(&c);

                if z.magnitude() > 2.0 {
                    break;
                }

            }

            let r = (iteration % 256) as u8;
            let g = ((iteration * 2) % 256) as u8;
            let b = ((iteration * 5) % 256) as u8;
            let pixel = Rgb([r, g, b]);
            img.put_pixel(x, y, pixel);
        }
    }

    img.save("fractal.png");
    
    // Measure elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Mandelbrot set generated in {} seconds and {} milliseconds. or {} minutes", elapsed_time.as_secs(), elapsed_time.subsec_millis(), elapsed_time.as_secs() / 60);
}
