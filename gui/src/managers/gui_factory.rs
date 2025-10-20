use crate::widgets::button::Button;
use crate::widgets::clickable::Clickable;
use sdl2::ttf::Font;
use sdl2::render::{TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas};
use crate::widgets::utils::CustomEvent;

pub struct GuiFactory<'a, 'b, 'c>{
    button_collection : Vec<Button<'a, 'b, 'c>>
}

impl<'a, 'b, 'c> GuiFactory<'a, 'b, 'c> {
    pub fn new() -> Self {
        Self {
            button_collection: Vec::new()
        }
    }
    pub fn create_button(& mut self, text: &str, color: sdl2::pixels::Color, 
        width: i32, height: i32, x: i32, y: i32, 
        font : &'a Font<'b, 'c>, texture_creator : &'a TextureCreator<WindowContext>){
            let button = match Button::new(text, color, width, height, x, y, font, texture_creator) {
                Ok(btn ) => btn,
                Err(e) => {
                    eprintln!("Error when creating button : {:?}", e);
                    return;
                }
            };
            self.button_collection.push(button);
    }

    pub fn get_buttons(& self) -> &Vec<Button>{
        &self.button_collection
    }

    pub fn draw(& self, canvas: &mut Canvas<Window>) {
        for btn in &self.button_collection{
            btn.draw(canvas);
        }
    }
    pub fn handle_event(& mut self, custom_event: &CustomEvent){
        for btn in & mut self.button_collection{
            btn.handle_event(custom_event);
        }
    }
}