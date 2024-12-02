
extern crate sdl2;

use sdl2::clear_error;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use std::sync::mpsc::RecvError;
use std::time::Duration;
use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH};

const TEXTURE_SIZE:u32 = 32;
const CHANGE_COLOR_FREQUENCY:u64 = 5;

#[derive(Clone, Copy)]
enum TextureColor{
    Green,
    Blue,
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL Initialisation failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .expect("Failed to create window");

    let mut canvas = window.into_canvas().build().expect("Failed to convert window into canvas");
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut green_square: Texture = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Green, TEXTURE_SIZE).expect("Failed to create green square texture");
    let mut blue_square = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TEXTURE_SIZE).expect("Failed to create blue square texture");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    let now = SystemTime::now();
    let mut current_epoch = match now.duration_since(UNIX_EPOCH) {
        Ok(value) => value.as_secs(),
        Err(e) => 0 as u64,
    };
    let mut tmp_first_square_texture: &Texture = &green_square;
    let mut tmp_second_square_texture: &Texture = &blue_square;
    let mut should_change: bool = true;
    'running: loop{
        for event in event_pump.poll_iter() {
            match  event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => 
                {
                    break 'running;
                },
                _ => {}    
            }
        }
        let now_inner = SystemTime::now();
        let epoch_to_change = match now_inner.duration_since(UNIX_EPOCH) {
            Ok(value) => value.as_secs(),
            Err(e) => 0 as u64,
        };
        
        if epoch_to_change - current_epoch == CHANGE_COLOR_FREQUENCY{
            println!("SHOULD CHANGE");
            if should_change {
                tmp_first_square_texture = &blue_square;
                tmp_second_square_texture = &green_square;
                should_change = false;
            }
            else{
                tmp_first_square_texture = &green_square;
                tmp_second_square_texture = &blue_square;
                should_change = true;
            }
            current_epoch = epoch_to_change;
        }
        
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.copy(tmp_first_square_texture, None, Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE)).expect("Failed to copy texture in to canvas");
        canvas.copy(tmp_second_square_texture, None, Rect::new(TEXTURE_SIZE as i32, TEXTURE_SIZE as i32, TEXTURE_SIZE, TEXTURE_SIZE)).expect("Failed to copy texture in to canvas");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
texture_creator: &'a TextureCreator<WindowContext>,
color: TextureColor, size : u32) -> Option<Texture<'a>>{
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size){
        _ = canvas.with_texture_canvas(&mut square_texture, |texture|{
            match color {
                TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255,0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0,255)),
            }
            texture.clear();
        }).expect("Failed to color texture");
        Some(square_texture)
    }
    else {
        None
    }
} 