use imgui::ColorStackToken;
use crate::game::player::Player;
use crate::widgets::colors::*;

pub struct Button {
    name: String,
    width: f32,
    height: f32,
    on_click: Box<dyn FnMut()>,
    color: Option<[[f32; 4]; 3]>
}
impl Button {
    pub fn new(_name: String, _width: f32, _height: f32, _on_click: Box<dyn FnMut()> ) -> Button {
        Button{
            name: _name,
            width: _width,
            height: _height,
            on_click: _on_click,
            color: None
        }
    }

    fn init(&mut self){
        if self.color == None{
            self.color = Some([RED, RED_HOVERD, RED_ACTIVE]);
        }
    }

    pub fn draw(&mut self, ui: &imgui::Ui){
        self.init();

        let color:[ColorStackToken;3] = [ui.push_style_color(imgui::StyleColor::Button, self.color.unwrap()[0]),
        ui.push_style_color(imgui::StyleColor::ButtonHovered, self.color.unwrap()[1]), // Lighter red when hovered
        ui.push_style_color(imgui::StyleColor::ButtonActive, self.color.unwrap()[2])]; // Darker red when clicked

        if ui.button_with_size(&self.name, [self.width, self.height]){
            (self.on_click)();
        }
        for item  in color {
            item.pop();
        }
    }

    pub fn set_color(&mut self, color_package: [[f32; 4]; 3]){
        self.color = Some(color_package);
    }
}