use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
// use sdl2::sys::Font;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
// use sdl2::render::TextureQuery;
use super::clickable::Clickable;

pub struct Button<'a>{
    text: String,
    color: sdl2::pixels::Color,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    is_touched : bool,
    is_hovered : bool,
    color_hover: Color,
    btn_rect : Rect,
    text_texture : Option<Texture<'a>>,
    text_rect : Option<Rect>,
}


impl<'a> Button<'a> {
    pub fn new(text: &str, color: sdl2::pixels::Color, 
        width: i32, height: i32, x: i32, y: i32, 
        font : &Font, texture_creator : &'a TextureCreator<WindowContext>) -> Result<Self, String>{
        let mut button = Self {
            text : text.to_string(),
            color : color,
            width : width,
            height : height,
            x : x,
            y : y,
            is_touched : false,
            is_hovered : false,
            color_hover: Color::RGB(173, 235, 179),
            btn_rect : Rect::new(x, y, width as u32, height as u32),
            text_texture : None,
            text_rect : None,
        };

        let _ = button.prepare_text(font, texture_creator);
        Ok(button)
        
    }

    pub fn draw(& self, canvas: &mut Canvas<Window>) -> Result<(), String>{
        if !self.is_hovered{
            canvas.set_draw_color(self.color);
        }
        else{
            canvas.set_draw_color(self.color_hover);
        }
        let _ = canvas.fill_rect(self.btn_rect);

        if let (Some(texture), Some (rect)) = (&self.text_texture, self.text_rect){
            canvas.copy(texture, None, rect)?;
        }

        Ok(())
    }

    fn prepare_text(&mut self, font : &Font, texture_creator : &'a TextureCreator<WindowContext>,)-> Result<(), String>{
        let surface = font.render(self.text.as_str()).blended(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string());
        let (w, h) = surface.size();
        let rect = Rect::new(
            self.x + (self.width - w as i32) / 2,
            self.y + (self.height - h as i32) / 2,
            w,
            h,
        );
        self.text_texture = Some(texture?);
        self.text_rect = Some(rect);
        Ok(())
    }

    //TODO create handle_event function to combine on_touch -n_hover 
}

impl<'a> Clickable for Button<'a> {
    fn on_touch(& mut self, touch_point: & Point) -> bool{
        if touch_point.x > self.x && 
            touch_point.x < self.x + self.width && 
            touch_point.y > self.y &&
            touch_point.y < self.y + self.height{
                println!("touch");
                self.is_touched = true;
                return true;
            }
            self.is_touched = false;
            return false
    }

    fn on_hover(& mut self, touch_point: & Point) -> bool{
        if touch_point.x > self.x && 
            touch_point.x < self.x + self.width && 
            touch_point.y > self.y &&
            touch_point.y < self.y + self.height{
                self.is_hovered = true;
                return true;
            }
            self.is_hovered = false;
            return false
    }
}