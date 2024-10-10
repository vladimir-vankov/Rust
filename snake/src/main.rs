extern crate sdl2;
mod const_vars;
mod game_context;
mod renderer;
use renderer::Renderer;
use game_context::GameContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use std::time::Duration;
use const_vars::*;


pub fn main() -> Result<(), String> {
    let sdl_context = match sdl2::init() {
        Ok(result_context) => result_context,
        Err(e) =>{
            return  Err(e);
        }
    };
    let video_subsystem = match sdl_context.video() {
        Ok(result_subsystem) => result_subsystem,
        Err(e) => {
            return  Err(e);
        }
    };

    let window = video_subsystem
    .window("Snake Game", (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32, (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window)?;

    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'running,
                _ => {}
            }
        }
        let _ = renderer.draw(&context);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
