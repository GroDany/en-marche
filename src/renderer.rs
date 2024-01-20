use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl};

use crate::marcher::Marcher;

pub struct Renderer {
    width: u32,
    height: u32,
    sdl_context: Sdl,
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
}
