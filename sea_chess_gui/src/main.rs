extern crate sdl2;
use imgui::ColorStackToken;
use imgui::ItemWidthStackToken;
// extern crate imgui_glow_renderer;
// use sdl2::sys::{SDL_SetWindowResizable, SDL_bool};
use imgui::WindowFlags;
use imgui::Context;
use imgui_glow_renderer::{
    glow::{self, HasContext},
    AutoRenderer,
};
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
    event::{Event, WindowEvent},
    video::{GLProfile, Window},
};

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn main() {
    /* initialize SDL and its video subsystem */
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    /* hint SDL to initialize an OpenGL 3.3 core profile context */
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);
    let mut window_width:u32 = 800;
    let mut window_height:u32 = 600;
    /* create a new window, be sure to call opengl method on the builder when using glow! */
    let window = video_subsystem
        .window("Sea Chess", window_width, window_height)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // let raw_window = window.raw();
    // unsafe {
    //     SDL_SetWindowResizable(raw_window, SDL_bool::SDL_TRUE);
    // }

    println!("Window flags: {:?}", window.window_flags());
    /* create a new OpenGL context and make it current */
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    /* enable vsync to cap framerate */
    window.subsystem().gl_set_swap_interval(1).unwrap();

    /* create new glow and imgui contexts */
    let gl = glow_context(&window);

    /* create context */
    let mut imgui = Context::create();

    /* disable creation of files on disc */
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    /* setup platform and renderer, and fonts to imgui */
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    /* create platform and renderer */
    let mut platform = SdlPlatform::new(&mut imgui);
    let mut renderer = AutoRenderer::new(gl, &mut imgui).unwrap();
    
    /* start main loop */
    let mut event_pump = sdl.event_pump().unwrap();
    let mut show = false;
    let mut show_once: bool = false;
    'main: loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            platform.handle_event(&mut imgui, &event);
            match event {
                Event::Window { win_event, .. } => match win_event{
                    WindowEvent::Resized(width, height) => {
                        window_width = width as u32;
                        window_height = height as u32;
                        println!("Window width {}, height {}", window_width, window_height);
                    },
                    _ => {}
                }
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();
        let mut player_one = Player::new("Vladimir".to_string(), 'X');
        if let Some(wt) = ui.window("Example window")
        .size([window_width as f32, window_height as f32], imgui::Condition::Always)
        .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_RESIZE)
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .begin()
        {
            if ui.button_with_size("Click me:", [100.0, 50.0]){
                show = !show;
                show_once = true;
            }
            let button_one = Button::new("test button".to_string(), 
                                                100.0,
                                                50.0,
                                                print_player);
            let button_two = Button::new("test button 2".to_string(), 
                                                100.0,
                                                50.0,
                                                print_player);
            button_one.draw(ui, &mut player_one);
            ui.same_line();
            button_two.draw(ui, &mut player_one);
            if show {
                if show_once {
                    show_once = false;
                }
                ui.text("WINDOW IS VISIBLE ".to_string() + &player_one.name);
            }
            wt.end();
        }
        /* render */
        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();
    }
}

struct Player {
    name: String,
    symbol: char,
}
const RED: [f32;4] = [1.0, 0.0, 0.0, 1.0];
const RED_HOVERD: [f32;4] = [1.0, 0.5, 0.5, 1.0];
const RED_ACTIVE: [f32;4] = [0.8, 0.0, 0.0, 1.0];
struct Button {
    name: String,
    width: f32,
    height: f32,
    on_click: fn(&mut Player),
    color: Option<[[f32; 4]; 4]>
}
impl Button {
    pub fn new(_name: String, _width: f32, _height: f32, _on_click: fn(&mut Player) ) -> Button {
        Button{
            name: _name,
            width: _width,
            height: _height,
            on_click: _on_click,
            color: None
        }
    }

    pub fn draw(&self, ui: &imgui::Ui, player: &mut Player){
        let custom_color = RED; // RGBA (Red)

        let color:[ColorStackToken;3] = [ui.push_style_color(imgui::StyleColor::Button, custom_color),
        ui.push_style_color(imgui::StyleColor::ButtonHovered, RED_HOVERD), // Lighter red when hovered
        ui.push_style_color(imgui::StyleColor::ButtonActive, RED_ACTIVE)]; // Darker red when clicked

        if ui.button_with_size(&self.name, [self.width, self.height]){
            (self.on_click)(player);
        }
        for item  in color {
            item.pop();
        }
    }
}

fn print_player(player: &mut Player){
    println!("Player name : {} \nPlayer Symbol : {}",player.name, player.symbol);
}

impl Player {
    pub fn new(_name:String, _symbol:char)-> Player {
        Player{
            name : _name,
            symbol : _symbol,
        }
    }
}