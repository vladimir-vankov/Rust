extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

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
    .window("First Rust created window", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    //choose color for window background
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    //fill window
    canvas.clear();
    //Draw color
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
