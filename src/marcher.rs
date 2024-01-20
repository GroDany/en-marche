use crate::{
    pixel::Pixel,
    shapes::{Point, Sphere},
};

const NUMBER_OF_STEPS: u8 = 32;

pub struct Marcher {
    pixels: Vec<Pixel>,
    rays: Vec<Point>,
    // TEMP shape for testing will be replace with a vector
    shape: Sphere,
    width: u32,
    height: u32,
}

impl Marcher {
    pub fn new(width: u32, height: u32) -> Self {
        let mut pixels: Vec<Pixel> = Vec::new();
        pixels.reserve((width * height) as usize);
        let mut rays: Vec<Point> = Vec::new();
        rays.reserve((width * height) as usize);

        let shape = Sphere {
            center: Point {
                x: 0.0,
                y: 20.0,
                z: 0.0,
            },
            radius: 1.0,
        };

        Self {
            pixels,
            rays,
            shape,
            width,
            height,
        }
    }

    pub fn init_rays(&mut self) {
        let mut x_og: f64 = -1.0 * (self.width as f64 / 2.0);
        let mut z_og: f64 = self.height as f64 / 2.0;

        for i in 0..self.height {
            for j in 0..self.width {
                let point = Point {
                    x: x_og + j as f64,
                    y: 10.0,
                    z: z_og + i as f64,
                };
                self.rays.push(point.norm());
            }
        }
    }
}
