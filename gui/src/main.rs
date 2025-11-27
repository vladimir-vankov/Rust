extern crate sdl2;
use std::cell::RefCell;
use std::time::Duration;

pub const GRID_X_SIZE: i32 = 40;
pub const GRID_Y_SIZE: i32 = 30;
pub const DOT_SIZE_IN_PXS: i32 = 20;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::mouse::MouseButton;
use std::collections::VecDeque;
use std::rc::Rc;

mod widgets;
use widgets::{clickable::Clickable, button::Button, text_input::TextInput};
use widgets::utils::CustomEvent;
use widgets::utils::EventType;

mod managers;
use managers::font_manager::FontManger;
use managers::gui_factory::GuiFactory;

use crate::widgets::observer;



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

    let font_manager = FontManger::new()?;

    let window = video_subsystem
    .window("Test Window", (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32, (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32)
    .position_centered()
    .resizable()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = Rc::new(canvas.texture_creator());

    let running = Rc::new(RefCell::new(true));
    let running_clone: Rc<RefCell<bool>> = running.clone();

    let mut event_pump = sdl_context.event_pump()?;

    let mut main_factory = GuiFactory::new();
    main_factory.create_button("Enter", 
                                                                    sdl2::pixels::Color::RGB(41, 74, 122), 
                                                                    300, 100, 200, 100,
                                                                font_manager.get_font(24).unwrap(),
                                                                &texture_creator);
    main_factory.create_button("Quit", 
                                                                    sdl2::pixels::Color::RGB(41, 74, 122),
                                                                    300, 100, 200, 250,
                                                                font_manager.get_font(24).unwrap(),
                                                                &texture_creator);
    main_factory.create_text_input("Please Enter text ...", 
                                                                    sdl2::pixels::Color::RGB(34, 51, 75),
                                                                    Rect::new(200, 400, 400, 50),
                                                                font_manager.get_font(16).unwrap(),
                                                                &texture_creator);
    
    if let Some(quit_btn) = main_factory.get_button("Quit".to_string()){
        let quit_clone: Rc<RefCell<Button>> = quit_btn.clone();
        quit_clone.borrow_mut().events().subscribe(observer::Event::Click, Rc::new(RefCell::new(move || {
                *running_clone.borrow_mut() = false;
            }))
        );
    }
    let mut events_queue : VecDeque<CustomEvent> = VecDeque::new();
    'running: loop {
        if !*running.borrow() {
            break 'running;
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::TextInput { text, .. } => {
                    if text != ""{
                        println!("Text input: {}", text);
                        events_queue.push_back(CustomEvent { event_type: EventType::TextInput, point: Point::new(0, 0), text : text});
                    }
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            events_queue.push_back(CustomEvent { event_type: EventType::Touch, point: Point::new(x, y), text : "".to_string() });
                        },
                        MouseButton::Right => println!("right click ({}, {})", x, y),
                        MouseButton::Middle => println!("middle click ({}, {})", x, y),
                        _ => println!("button {:?} : ({}, {})", mouse_btn, x, y)
                    }
                }
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    println!("Released {:?} : ({}, {})", mouse_btn, x, y);
                    events_queue.push_back(CustomEvent { event_type: EventType::UnTouch, point: Point::new(x, y), text : "".to_string() });
                }
                _ => {}
            }
        }
        
        let mouse_state = event_pump.mouse_state();
        events_queue.push_back(CustomEvent { event_type: EventType::Hover, point: Point::new(mouse_state.x(), mouse_state.y()), text : "".to_string() });
        
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        
        canvas.clear();
        
        main_factory.draw(&mut canvas);
        while let Some(custom_event) = events_queue.pop_front(){
            main_factory.handle_event(&custom_event);
        }
        
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
