use crate::widgets::button::Button;
use crate::widgets::clickable::Clickable;
use crate::widgets::text_input::TextInput;
use sdl2::ttf::Font;
use sdl2::render::{TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas};
use sdl2::rect::Rect;
use std::rc::Rc;
use std::cell::RefCell;
use crate::widgets::utils::CustomEvent;

pub struct GuiFactory<'a>{
    button_collection : Vec<Rc<RefCell<Button<'a>>>>,
    text_input_collection : Vec<Rc<RefCell<TextInput<'a>>>>
}

impl<'a> GuiFactory<'a> {
    pub fn new() -> Self {
        Self {
            button_collection: Vec::new(),
            text_input_collection: Vec::new()
        }
    }
    pub fn create_button(& mut self, text: &str, color: sdl2::pixels::Color, 
        width: i32, height: i32, x: i32, y: i32, 
        font : Rc<RefCell<Font<'static, 'static>>>, texture_creator : &'a TextureCreator<WindowContext>){
            match Button::new(text, color, width, height, x, y, font, texture_creator) {
                Ok(btn ) =>{
                    self.button_collection.push(Rc::new(RefCell::new(btn)));
                },
                Err(e) => {
                    eprintln!("Error when creating button : {:?}", e);
                    return;
                }
            };
    }

    pub fn create_text_input(& mut self, text: &str, color: sdl2::pixels::Color, btn_rect: Rect, 
        font : Rc<RefCell<Font<'static, 'static>>>, texture_creator : &'a TextureCreator<WindowContext>){
        let text_input = match TextInput::new(text, color, btn_rect, font, texture_creator) {
            Ok(input_field ) => input_field,
            Err(e) => {
                eprintln!("Error when creating TextInput field : {:?}", e);
                return;
            }
        };
        self.text_input_collection.push(Rc::new(RefCell::new(text_input)));
    }

    // pub fn get_buttons(& self) -> &Vec<Rc<RefCell<Button>>>{
    //     &self.button_collection
    // }

    pub fn get_button(&mut self, name: String) -> Option<Rc<RefCell<Button<'a>>>>{
        self.button_collection
        .iter()
        .find(|btn| btn.borrow().text == name)
        .cloned()//increase shared counter
    }

    pub fn draw(& self, canvas: &mut Canvas<Window>) {
        for btn in &self.button_collection{
            let _ = btn.borrow().draw(canvas);
        }
        for input_field in &self.text_input_collection{
            let _ = input_field.borrow().draw(canvas);
        }
    }

    pub fn handle_event(& mut self, custom_event: &CustomEvent){
        for btn in & mut self.button_collection{
            btn.borrow_mut().handle_event(custom_event);
        }
        for input_field in & mut self.text_input_collection{
            input_field.borrow_mut().handle_event(custom_event);
        }
    }
}