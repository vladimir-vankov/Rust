use sdl2::rect::Point;
use super::utils::CustomEvent;
use super::utils::EventType;

pub trait Clickable {
    fn on_touch(& mut self, touch_point: & Point) -> bool;
    
    fn on_un_touch(& mut self, touch_point: & Point) -> bool{
        println!("UnTouch ({}, {})", touch_point.x, touch_point.y);
        true
    }
    
    fn on_hover(& mut self, touch_point: & Point) -> bool;
    
    fn handle_event(& mut self, custom_event: &CustomEvent){
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
