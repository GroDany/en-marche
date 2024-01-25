use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window, Sdl};

use crate::{marcher::Marcher, pixel::Pixel};

pub struct Renderer {
    width: u32,
    height: u32,
    // TEMP: pub
    pub sdl_context: Sdl,
    canvas: Canvas<Window>,
    marcher: Marcher,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("En Marche", width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let mut marcher = Marcher::new(width, height);
        marcher.init_rays();

        Ok(Self {
            width,
            height,
            sdl_context,
            canvas,
            marcher,
        })
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn run(&mut self) {
        let pixels = self.marcher.march();
        let mut color = pixels[0].color;
        let mut pixels_to_draw: Vec<&Pixel> = vec![];
        self.canvas.set_draw_color(color);
        for pixel in pixels.iter() {
            if pixel.color != color {
                let _ = self.canvas.draw_points(
                    pixels_to_draw
                        .iter()
                        .map(|x| x.coord)
                        .collect::<Vec<Point>>()
                        .as_slice(),
                );
                color = pixel.color;
                self.canvas.set_draw_color(color);
                pixels_to_draw.clear();
            } else {
                pixels_to_draw.push(pixel);
            }
        }
        self.canvas.present();
    }
}
