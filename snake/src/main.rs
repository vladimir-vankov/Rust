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
use std::time::Instant;

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

    let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
    
    ////////////////////////
    let mut event_pump = sdl_context.event_pump()?;
    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window)?;
    let mut frame_counter = 0;
    
    

    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), ..} => {
                    match keycode  {
                        Keycode::W | Keycode::Up => context.move_up(),
                        Keycode::S | Keycode::Down => context.move_down(),
                        Keycode::A | Keycode::Left => context.move_left(),
                        Keycode::D | Keycode::Right => context.move_right(),
                        Keycode::Escape => context.toggle_pause(),
                        _ => {}
                    }
                },
                _ => {},
            }
        }
        frame_counter += 1;
        if frame_counter % 10 == 0 {
            context.next_tick();
            frame_counter = 0;
        }
        match renderer.draw(&context) {
            Ok(()) => {},
            Err(e) => println!("There is been error : {}", e)
        }

        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
