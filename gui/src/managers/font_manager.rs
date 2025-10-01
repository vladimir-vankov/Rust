use std::collections::HashMap;
use sdl2::ttf::Font;

pub struct FontManger<'ttf, 'surf>{
    fonts : HashMap<u16, Font<'ttf, 'surf>>,
}

impl <'ttf, 'surf> FontManger<'ttf, 'surf> {
    pub const FONT_PATH:&'static str = "fonts/Roboto-ExtraBold.ttf";

    pub fn new(ttf_context: &'ttf sdl2::ttf::Sdl2TtfContext) -> Result<Self, String> {
        let mut fonts = HashMap::new();
        fonts.insert(16, ttf_context.load_font(Self::FONT_PATH, 16).map_err(|e| e.to_string())?);
        fonts.insert(24, ttf_context.load_font(Self::FONT_PATH, 24).map_err(|e| e.to_string())?);
        fonts.insert(32, ttf_context.load_font(Self::FONT_PATH, 32).map_err(|e| e.to_string())?);

        Ok(Self { fonts })
    }

    pub fn get_font(&self, size:u16) -> Option<&Font<'ttf, 'surf>>{
        self.fonts.get(&size)
    }
}