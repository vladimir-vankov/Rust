use sdl2::{rect::Point};
use sdl2::pixels::Color;
use sdl2::ttf::Font;

pub enum EventType{
    Touch,
    UnTouch,
    Hover
}

pub struct CustomEvent{
    pub event_type: EventType,
    pub point : Point
}

pub fn get_text_size(font: &Font, text : &str) -> Result<(u32, u32), String>{
    let surface = font.render(text).blended(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;    
    Ok(surface.size())
}