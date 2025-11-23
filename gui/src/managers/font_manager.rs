use std::collections::HashMap;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::cell::RefCell;

pub struct FontManger<'ttf, 'surf>{
    fonts : HashMap<u16, Rc<RefCell<Font<'ttf, 'surf>>>>,
}

impl <'ttf, 'surf> FontManger<'ttf, 'surf> {
    pub const FONT_PATH:&'static str = "fonts/Roboto-ExtraBold.ttf";

    pub fn new() -> Result<Self, String> {
        let ttf_context = sdl2::ttf::init().unwrap();
        let ttf_context: &'static sdl2::ttf::Sdl2TtfContext = Box::leak(Box::new(ttf_context));
        let mut fonts = HashMap::new();
        fonts.insert(16, Rc::new(RefCell::new(ttf_context.load_font(Self::FONT_PATH, 16).map_err(|e| e.to_string())?)));
        fonts.insert(24, Rc::new(RefCell::new(ttf_context.load_font(Self::FONT_PATH, 24).map_err(|e| e.to_string())?)));
        fonts.insert(32, Rc::new(RefCell::new(ttf_context.load_font(Self::FONT_PATH, 32).map_err(|e| e.to_string())?)));

        Ok(Self { fonts })
    }

    pub fn get_font(&self, size:u16) -> Option<Rc<RefCell<Font<'ttf, 'surf>>>>{
        self.fonts.get(&size).cloned()
    }
}