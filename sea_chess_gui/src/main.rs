extern crate sdl2;

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

mod widgets;
use widgets::button::Button as Button;
mod game;
use game::player::Player as Player;
use widgets::colors::BUTTON_HEIGHT;
use widgets::colors::BUTTON_WIDTH;

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
    let mut show:bool = true;
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
        if let Some(wt) = ui.window("Example window")
        .size([window_width as f32, window_height as f32], imgui::Condition::Always)
        .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_RESIZE | WindowFlags::NO_MOVE)
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .begin()
        {
            let button_width:f32 = window_width as f32 / 3.0;
            let button_height:f32 = BUTTON_HEIGHT + 20.0;
            if show{
                println!("cursor screen pos : {:?}", ui.cursor_screen_pos());
                show = false;
            }
            buttons_align(ui, &(window_width as f32), &button_width);
            if ui.button_with_size("NEW GAME", [button_width, button_height]){
                println!("NEW GAME");
            }
            buttons_align(ui, &(window_width as f32), &button_width);
            if ui.button_with_size("LAN MULTIPLAYER", [button_width, button_height]){
                println!("LAN MULTIPLAYER");
            }
            buttons_align(ui, &(window_width as f32), &button_width);
            if ui.button_with_size("HIGHSCORES", [button_width, button_height]){
                println!("HIGHSCORES");
            }
            buttons_align(ui, &(window_width as f32), &button_width);
            if ui.button_with_size("EXIT", [button_width, button_height]){
                println!("EXIT");
                break 'main;
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

//It should not work but because of imgui implementation it does
fn buttons_align(ui: &imgui::Ui, window_width: &f32, button_width: &f32){
    let current_cursor_pos: [f32; 2] = ui.cursor_screen_pos();
    ui.set_cursor_pos([(window_width/2.0 - button_width/2.0), current_cursor_pos[1]]);
}


fn print_player(player: &mut Player){
    println!("Player name : {} \nPlayer Symbol : {}",player.name, player.symbol);
}

