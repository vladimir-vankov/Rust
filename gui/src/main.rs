extern crate sdl2;
use std::time::Duration;

pub const GRID_X_SIZE: i32 = 40;
pub const GRID_Y_SIZE: i32 = 30;
pub const DOT_SIZE_IN_PXS: i32 = 20;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{self, Point};
use sdl2::mouse::MouseButton;
use sdl2::render::Canvas;
// use sdl2::sys::Window;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::collections::VecDeque;


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
    .window("Test Window", (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32, (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32)
    .position_centered()
    .resizable()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut first_button = Button::new("Test", sdl2::pixels::Color::RGB(255, 0, 0), 300, 100, 200, 200); 
    let mut events_queue : VecDeque<CustomEvent> = VecDeque::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            events_queue.push_back(CustomEvent { event_type: EventType::Touch, point: Point::new(x, y) });
                        },
                        MouseButton::Right => println!("right click ({}, {})", x, y),
                        MouseButton::Middle => println!("middle click ({}, {})", x, y),
                        _ => println!("button {:?} : ({}, {})", mouse_btn, x, y)
                    }
                }
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    println!("Released {:?} : ({}, {})", mouse_btn, x, y);
                    events_queue.push_back(CustomEvent { event_type: EventType::UnTouch, point: Point::new(x, y) });
                }
                _ => {}
            }
        }
        
        let mouse_state = event_pump.mouse_state();
        events_queue.push_back(CustomEvent { event_type: EventType::Hover, point: Point::new(mouse_state.x(), mouse_state.y()) });
        
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        
        canvas.clear();
        
        first_button.draw(&mut canvas);
        while let Some(custom_event) = events_queue.pop_front(){
            first_button.handle_event(custom_event);
        }
        
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

enum EventType{
    Touch,
    UnTouch,
    Hover
}

struct CustomEvent{
    event_type: EventType,
    point : Point
}

struct Button{
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

trait clickable {
    fn on_touch(& mut self, touch_point: & Point) -> bool;
    fn on_unTouch(& mut self, touch_point: & Point) -> bool{
        println!("UnTouch ({}, {})", touch_point.x, touch_point.y);
        true
    }
    fn on_hover(& mut self, touch_point: & Point) -> bool;
    fn handle_event(& mut self, custom_event:CustomEvent){
        match custom_event.event_type {
            EventType::Touch => {
                _ = self.on_touch(&custom_event.point)
            },
            EventType::Hover => {
                _ = self.on_hover(&custom_event.point)
            },
            _ => {}
        }
    }
}

impl Button {
    fn new(text: &str, color: sdl2::pixels::Color, width: i32, height: i32, x: i32, y: i32) -> Self{
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

    fn draw(& self, canvas: &mut Canvas<Window>) -> Result<(), String>{
        if !self.is_hovered{
            canvas.set_draw_color(self.color);
        }
        else{
            canvas.set_draw_color(self.color_hover);
        }
        canvas.fill_rect(self.btn_rect);
        Ok(())
    }

    //TODO create handle_event function to combine on_touch -n_hover 
}

impl clickable for Button {
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