use sdl2::rect::Point;
pub enum EventType{
    Touch,
    UnTouch,
    Hover
}

pub struct CustomEvent{
    pub event_type: EventType,
    pub point : Point
}