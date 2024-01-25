use sdl2::{pixels::Color, rect::Point};

use crate::{
    pixel::Pixel,
    point::EMPoint,
    shapes::{Distance, Sphere},
};

const NUMBER_OF_STEPS: u8 = 32;
const HIT_RANGE: f64 = 5.0;
const MAXIMUM_DISTANCE: f64 = 0.001;

pub struct Marcher {
    pixels: Vec<Pixel>,
    rays: Vec<EMPoint>,
    width: u32,
    height: u32,
    // TEMP shape for testing will be replace with a vector
    shape: Sphere,
}

impl Marcher {
    pub fn new(width: u32, height: u32) -> Self {
        let mut pixels: Vec<Pixel> = Vec::new();
        pixels.reserve((width * height) as usize);
        let mut rays: Vec<EMPoint> = Vec::new();
        rays.reserve((width * height) as usize);

        let shape = Sphere {
            center: EMPoint {
                x: 0.0,
                y: 20.0,
                z: 0.0,
            },
            radius: 10.0,
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
        let x_og: f64 = -1.0 * (self.width as f64 / 2.0);
        let z_og: f64 = self.height as f64 / 2.0;

        for i in 0..self.height {
            for j in 0..self.width {
                let point = EMPoint {
                    x: x_og + j as f64,
                    y: 10.0,
                    z: z_og - i as f64,
                };
                // println!("x{}, y{}, z{}", point.x, point.y, point.z);
                self.rays.push(point.norm());
            }
        }
    }

    pub fn march(&mut self) -> &Vec<Pixel> {
        self.march_all();
        // for pixel in self.pixels.iter() {
        //     println!("x: {}, y: {}", pixel.coord.x, pixel.coord.x);
        // }
        return &self.pixels;
    }

    fn march_all(&mut self) {
        for (pos, e) in self.rays.iter().enumerate() {
            let x: i32 = pos as i32 % self.width as i32;
            let y: i32 = pos as i32 / self.width as i32;
            let coord: Point = Point::new(x, y);
            // println!("x{}, y{}", x, y);
            // println!("x{}, y{}, z{}", e.x, e.y, e.z);
            if self.marche_one(e) {
                self.pixels
                    .push(Pixel::new(coord, Color::RGB(255, 255, 255)));
            } else {
                self.pixels.push(Pixel::new(coord, Color::RGB(0, 0, 0)));
            }
        }
    }

    fn marche_one(&self, ray: &EMPoint) -> bool {
        let mut travelled: f64 = 0.0;

        let mut current_position = EMPoint::from(ray);
        for _ in 0..NUMBER_OF_STEPS {
            // TEMP: Check the sphere only
            let distance = self.shape.distance(&current_position);

            // if distance < 5.0 {
            //     println!("distance {}", distance);
            // }

            if distance < HIT_RANGE {
                return true;
            }

            if travelled > MAXIMUM_DISTANCE {
                return false;
            }

            travelled += distance;
            current_position = current_position.mult_scalar(travelled);
        }

        false
    }
}
