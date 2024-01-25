use std::time::Duration;

use renderer::Renderer;
use sdl2::{event::Event, keyboard::Keycode};

mod marcher;
mod pixel;
mod point;
mod renderer;
mod shapes;

extern crate sdl2;

fn main() -> Result<(), String> {
    // Here add command line args parsing for config and options
    let mut renderer = Renderer::new(800, 600)?;
    renderer.clear();
    let mut event_pump = renderer
        .sdl_context
        .event_pump()
        .map_err(|e| e.to_string())?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        renderer.run();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
