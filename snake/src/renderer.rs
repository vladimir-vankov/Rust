extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use crate::const_vars::*; 
use crate::game_context::GameContext;
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

    fn draw_background(&mut self, context: &GameContext){
        let color = match context.state {
            GameState::Playing => Color::RGB (0, 0, 0),
            GameState::Paused => Color::RGB (30, 30, 30),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &context.player_position {
            self.draw_dot(point)?;
        }

        Ok(())
    }

    pub fn draw(&mut self, context:&GameContext) -> Result<(), String> {
        self.draw_background(&context);
        self.draw_player(&context)?;

        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&Point(3, 3))?;

        self.canvas.present();

        Ok(())
    } 
    // pub fn get_window(&mut self) -> &Window{
    //     self.canvas.window()
    // }
}