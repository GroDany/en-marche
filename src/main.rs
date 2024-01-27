use renderer::Renderer;
use sdl2::{event::Event, keyboard::Keycode};

mod marcher;
mod pixel;
mod point;
mod renderer;
mod shapes;

extern crate sdl2;

fn main() -> Result<(), String> {
    // TODO: Here add command line args parsing for config and options
    let mut renderer = Renderer::new(1920, 1080)?;
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
    }
    Ok(())
}
