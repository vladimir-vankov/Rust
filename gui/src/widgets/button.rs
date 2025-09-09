use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use super::clickable::Clickable;

pub struct Button{
    text: String,
    color: sdl2::pixels::Color,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    is_touched : bool,
    is_hovered : bool,
    color_hover: Color,
    btn_rect : Rect
}


impl Button {
    pub fn new(text: &str, color: sdl2::pixels::Color, width: i32, height: i32, x: i32, y: i32) -> Self{
        Self {
            text : text.to_string(),
            color : color,
            width : width,
            height : height,
            x : x,
            y : y,
            is_touched : false,
            is_hovered : false,
            color_hover: Color::RGB(173, 235, 179),
            btn_rect : Rect::new(x, y, width as u32, height as u32)
        }
    }

    pub fn draw(& self, canvas: &mut Canvas<Window>) -> Result<(), String>{
        if !self.is_hovered{
            canvas.set_draw_color(self.color);
        }
        else{
            canvas.set_draw_color(self.color_hover);
        }
        let _ = canvas.fill_rect(self.btn_rect);
        Ok(())
    }

    //TODO create handle_event function to combine on_touch -n_hover 
}

impl Clickable for Button {
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