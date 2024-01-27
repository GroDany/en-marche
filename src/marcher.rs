use sdl2::{pixels::Color, rect::Point};

use crate::{
    pixel::Pixel,
    point::EMPoint,
    shapes::{Distance, Sphere},
};

const NUMBER_OF_STEPS: u8 = 32;
const HIT_RANGE: f64 = 0.01;
const MAXIMUM_DISTANCE: f64 = 1000.0;
const SCREEN_Y: f64 = 5.0;

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
                y: 8.0,
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
        let x_og = -1.0 * (self.width as f64 / self.height as f64);
        let h_step = 2.0 / (self.height) as f64;

        for i in 0..self.height {
            for j in 0..self.width {
                let ray = EMPoint {
                    x: x_og + j as f64 * h_step,
                    y: SCREEN_Y,
                    z: 1.0 - i as f64 * h_step,
                }
                .unit();
                self.rays.push(ray);
            }
        }
    }

    pub fn march(&mut self) -> &Vec<Pixel> {
        self.march_all();
        return &self.pixels;
    }

    fn march_all(&mut self) {
        for (pos, e) in self.rays.iter().enumerate() {
            let x: i32 = pos as i32 % self.width as i32;
            let y: i32 = pos as i32 / self.width as i32;
            let coord: Point = Point::new(x, y);
            match self.march_one(e) {
                Some(color) => {
                    self.pixels.push({
                        let coord = coord;
                        Pixel { coord, color }
                    });
                }
                None => self.pixels.push(Pixel::new(coord, Color::RGB(0, 0, 0))),
            }
        }
    }

    fn march_one(&self, ray: &EMPoint) -> Option<Color> {
        let mut travelled: f64 = 0.0;
        let mut current_position = EMPoint {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        for _ in 0..NUMBER_OF_STEPS {
            let distance = self.map_scene(&current_position);
            if distance < HIT_RANGE {
                let light = EMPoint {
                    x: -10.0,
                    y: 0.0,
                    z: 10.0,
                };
                let normal = self.calculate_normal(&current_position);
                let light_d = light.sub_point(&current_position).unit();
                let dot = normal.dot(&light_d);
                let c = if dot > 0.0 { dot } else { 0.0 };
                let color = Color::RGB((0.0 * 255.0) as u8, (c * 255.0) as u8, (0.0 * 255.0) as u8);
                return Some(color);
            }
            if travelled > MAXIMUM_DISTANCE {
                return None;
            }
            travelled += distance;
            current_position = ray.mult_scalar(travelled);
        }

        None
    }

    fn map_scene(&self, point: &EMPoint) -> f64 {
        let test = (point.x * 4.0).sin() * (point.y * 4.0).sin() * (point.z * 4.0).sin() * 0.25;
        // TEMP: Checks the sphere only
        self.shape.distance(point) + test
    }

    fn calculate_normal(&self, point: &EMPoint) -> EMPoint {
        let x = self.map_scene(&point.add_x(0.001)) - self.map_scene(&point.sub_x(0.001));
        let y = self.map_scene(&point.add_y(0.001)) - self.map_scene(&point.sub_y(0.001));
        let z = self.map_scene(&point.add_z(0.001)) - self.map_scene(&point.sub_z(0.001));
        EMPoint { x, y, z }.unit()
    }
}
