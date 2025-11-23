use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use super::clickable::Clickable;
use super::observer::{Event, Publisher};
use std::rc::Rc;
use std::cell::RefCell;

// #[derive(Default)]
pub struct TextInput<'a>{
    text: String,
    color: sdl2::pixels::Color,
    is_hovered : bool,
    color_hover: Color,
    btn_rect : Rect,
    text_texture : Option<Texture<'a>>,
    text_rect : Option<Rect>,
    publisher: Publisher,
    font : Rc<RefCell<Font<'static, 'static>>>
}


impl<'a> TextInput<'a> {
    pub const BORDER_HEIGHT:u8 = 1;
    pub const PADDING:u8 = 5;

    pub fn new(text: &str, color: sdl2::pixels::Color, btn_rect: Rect, 
        font : Rc<RefCell<Font<'static, 'static>>>, texture_creator : &'a TextureCreator<WindowContext>) -> Result<Self, String>{
        let mut text_field = Self {
            text : text.to_string(),
            color : color,
            is_hovered : false,
            color_hover: Color::RGB(39, 73, 114),
            btn_rect : btn_rect,
            text_texture : None,
            text_rect : None,
            font : font,
            publisher : Publisher::default()
        };

        let _ = text_field.prepare_text( texture_creator );
        Ok(text_field)
    }

    pub fn draw(& self, canvas: &mut Canvas<Window>) -> Result<(), String>{
        let btn_frame = Rect::new(self.btn_rect.x + Self::BORDER_HEIGHT as i32,
                                            self.btn_rect.y + Self::BORDER_HEIGHT as i32,
                                            self.btn_rect.width() - (Self::BORDER_HEIGHT * 2) as u32, 
                                            self.btn_rect.height() - (Self::BORDER_HEIGHT * 2) as u32);
        //TODO bellow color is border color should be set to variable, may be constant
        canvas.set_draw_color(Color::RGB(72, 72, 72));
        let _ = canvas.fill_rect(self.btn_rect);

        if !self.is_hovered {
            canvas.set_draw_color(self.color);
        }
        else{
            canvas.set_draw_color(self.color_hover);
        }
        let _ = canvas.fill_rect(btn_frame);

        if let (Some(texture), Some (rect)) = (&self.text_texture, self.text_rect){
            canvas.copy(texture, None, rect)?;
        }

        Ok(())
    }

    fn prepare_text(&mut self, texture_creator : &'a TextureCreator<WindowContext>)-> Result<(), String>{
        let surface = self.font.borrow().render(self.text.as_str()).blended(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string());
        let (mut w, h) = surface.size();
        if w > self.btn_rect.width() - Self::PADDING as u32{
            w = self.btn_rect.width() - 2*Self::PADDING as u32;
        }
        let rect = Rect::new(
            self.btn_rect.x + Self::BORDER_HEIGHT as i32 + Self::PADDING as i32,
            self.btn_rect.y + (self.btn_rect.h - h as i32) / 2,
            w,
            h,
        );
        self.text_texture = Some(texture?);
        self.text_rect = Some(rect);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn events(&mut self) -> &mut Publisher{
        &mut self.publisher
    }

    pub fn notify(&mut self){
        self.publisher.notify(Event::Click);
    }

    #[allow(dead_code)]
    pub fn get_text_size(self) -> Result<(u32, u32), String>{
        let surface = self.font.borrow().render(self.text.as_str()).blended(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;    
        Ok(surface.size())
    }

    fn on_text_input_touch(&mut self) {
        println!("on_text_input_touch");
        self.text = "".to_string();
    }
    //TODO create handle_event function to combine on_touch -n_hover 
}

impl<'a> Clickable for TextInput<'a> {
    fn on_touch(& mut self, touch_point: & Point) -> bool{
        if touch_point.x > self.btn_rect.x && 
            touch_point.x < self.btn_rect.x + self.btn_rect.w && 
            touch_point.y > self.btn_rect.y &&
            touch_point.y < self.btn_rect.y + self.btn_rect.h{
                self.notify();
                self.on_text_input_touch();
                return true;
            }
            return false
    }

    fn on_hover(& mut self, touch_point: & Point) -> bool{
        if touch_point.x > self.btn_rect.x && 
            touch_point.x < self.btn_rect.x + self.btn_rect.w && 
            touch_point.y > self.btn_rect.y &&
            touch_point.y < self.btn_rect.y + self.btn_rect.h{
                self.is_hovered = true;
                return true;
            }
            self.is_hovered = false;
            return false
    }
}