use sdl2::{pixels::Color, rect::Point};

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub coord: Point,
    pub color: Color,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            coord: Point::new(0, 0),
            color: Color::RGB(0, 0, 0),
        }
    }
}

impl Pixel {
    pub fn new(coord: Point, color: Color) -> Self {
        Self { coord, color }
    }
}
