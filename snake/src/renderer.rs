extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use crate::game_context::Point;
use crate::const_vars::*;
pub struct Renderer { canvas : WindowCanvas }

impl Renderer {
    pub fn new(window: Window ) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String>{
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new((x * DOT_SIZE_IN_PXS) as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS as u32,
            DOT_SIZE_IN_PXS as u32,))?;
        Ok(())
    }
}