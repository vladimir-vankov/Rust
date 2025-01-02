
extern crate sdl2;
extern  crate rand;

// use sdl2::clear_error;
// use sdl2::libc::NOEXPR;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
// use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
// use core::time;
// use std::sync::mpsc::RecvError;
use std::time::Duration;
use std::thread::sleep;
use std::time::SystemTime;
use sdl2::image::{LoadTexture, InitFlag};

mod utils;
use utils::*;

const TEXTURE_SIZE:u32 = 32;
const CHANGE_COLOR_FREQUENCY:u64 = 5;
const GAME_TABLE_SIZE:u32 = 16;
const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];
const NB_HIGHSCORES: usize = 5;
const TETRIS_HEIGHT:usize = 40;
const HIGHSCORE_FILE: &'static str = "score.txt";

#[derive(Clone, Copy)]
enum TextureColor{
    Green,
    Blue,
    Black,
    White,
    Yellow,
    Red,
    Brown
}

//Implementing Tetrimino struct for the game.
type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

struct Tetrimino {
    states : States,
    x : isize,
    y : usize,
    current_state : u8,
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

struct TetriminoI;
impl TetriminoGenerator for TetriminoI{
    fn new() -> Tetrimino{
        Tetrimino {
            states: vec![vec![vec![1, 1, 1, 1],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![0, 1, 0, 0],
                            vec![0, 1, 0, 0],
                            vec![0, 1, 0, 0],
                            vec![0, 1, 0, 0]]],
            x : 4,
            y : 0,
            current_state : 0,
        }
    }
}

struct TetriminoJ;
impl TetriminoGenerator for TetriminoJ{
    fn new() -> Tetrimino{
        Tetrimino{
            states: vec![vec![vec![2, 2, 2, 0],
                                vec![2, 0, 0, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![2, 2, 0, 0],
                                vec![0, 2, 0, 0],
                                vec![0, 2, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![0, 0, 2, 0],
                                vec![2, 2, 2, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![2, 0, 0, 0],
                                vec![2, 0, 0, 0],
                                vec![2, 2, 0, 0],
                                vec![0, 0, 0, 0]]],
            x : 4,
            y : 0,
            current_state : 0,
        }
    }
}

struct TetriminoL;
impl TetriminoGenerator for TetriminoL{
    fn new() -> Tetrimino{
        Tetrimino{
            states: vec![vec![vec![3, 3, 3, 0],
                                vec![0, 0, 3, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![0, 3, 0, 0],
                                vec![0, 3, 0, 0],
                                vec![3, 3, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![3, 0, 0, 0],
                                vec![3, 3, 3, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![3, 3, 0, 0],
                                vec![3, 0, 0, 0],
                                vec![3, 0, 0, 0],
                                vec![0, 0, 0, 0]]],
            x : 4,
            y : 0,
            current_state : 0,
        }
    }
}

struct TetriminoO;
impl TetriminoGenerator for TetriminoO{
    fn new() -> Tetrimino {
        Tetrimino{
            states: vec![vec![vec![4, 4, 0, 0],
                            vec![4, 4, 0, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]]],
            x : 5,
            y : 0,
            current_state : 0,
        }
    }
}

struct TetriminoS;
impl TetriminoGenerator for TetriminoS{
    fn new() -> Tetrimino {
        Tetrimino{
            states: vec![vec![vec![0, 5, 5, 0],
                                vec![5, 5, 0, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![0, 5, 0, 0],
                                vec![0, 5, 5, 0],
                                vec![0, 0, 5, 0],
                                vec![0, 0, 0, 0]]],
            x : 4,
            y : 0,
            current_state : 0,
        }
    }
}

struct TetriminoZ;
impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino{
            states: vec![vec![vec![6, 6, 0, 0],
                                vec![0, 6, 6, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![0, 0, 6, 0],
                                vec![0, 6, 6, 0],
                                vec![0, 6, 0, 0],
                                vec![0, 0, 0, 0]]],
            x : 4,
            y : 0,
            current_state : 0,
        }
    }
}

struct TetriminoT;
impl TetriminoGenerator for TetriminoT{
    fn new() -> Tetrimino {
        Tetrimino{
            states: vec![vec![vec![7, 7, 7, 0],
                                vec![0, 7, 0, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![0, 7, 0, 0],
                                vec![7, 7, 0, 0],
                                vec![0, 7, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![0, 7, 0, 0],
                                vec![7, 7, 7, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]],
                            vec![vec![0, 7, 0, 0],
                                vec![0, 7, 7, 0],
                                vec![0, 7, 0, 0],
                                vec![0, 0, 0, 0]]],
            x : 4,
            y : 0,
            current_state : 0,
        }
    }
}

//end of tetrimino implementation

fn create_new_tetrimino() -> Tetrimino{
    static mut PREV: u8 = 7;
    let mut random_nb = rand::random::<u8>() % 7;
    //SHOULD BE OTHER WAY
    if unsafe { PREV } == random_nb{
        random_nb = rand::random::<u8>() % 7;
    }
    unsafe { PREV = random_nb; }
    match random_nb {
        0 => TetriminoI::new(),
        1 => TetriminoJ::new(),
        2 => TetriminoL::new(),
        3 => TetriminoO::new(),
        4 => TetriminoS::new(),
        5 => TetriminoZ::new(),
        6 => TetriminoT::new(),
        _ => unreachable!("When we devide (%) by 7 would not be more then 6."),
    }
}

impl Tetrimino {
    fn rotate(&mut self, game_map: &[Vec<u8>]){
        // self.current_state += 1;
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len(){
            tmp_state = 0;
        }
        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter(){
            if self.test_position(game_map, tmp_state as usize, self.x + x, self.y) == true {
                self.current_state = tmp_state;
                self.x += *x;
                break;
            }
        }
    }

    fn test_position(&self, game_map: &[Vec<u8>], tmp_state: usize, x: isize, y: usize) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0 &&
                (y + decal_y >= game_map.len() ||
                x < 0 ||
                x as usize >= game_map[y + decal_y].len() ||
            game_map[y + decal_y][x as usize] != 0){
                return false;
                }
            }
        }
        return true;
    }

    fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: isize) -> bool {
        if self.test_position(game_map, self.current_state as usize, new_x, new_y as usize) == true {
            self.x = new_x as isize;
            self.y = new_y as usize;
            true
        }
        else{
            false
        }
    }

    fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool{
        self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }
}

struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>,
}

impl Tetris {
    fn new() -> Tetris{
        let mut game_map = Vec::new();
        for _ in 0..GAME_TABLE_SIZE{
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        Tetris{
            game_map: game_map,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }

    fn check_lines(&mut self){
        let mut y = 0;
        let mut score_add = 0;

        while y < self.game_map.len() {
            let mut complete = true;
            
            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }

            if complete == true {
                score_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }
        if self.game_map.len() == 0 {
            //A tetris
            score_add += 1000;
        }
        self.update_score(score_add);
        while self.game_map.len() < GAME_TABLE_SIZE as usize {
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    fn make_permanent(&mut self){
        let mut to_add = 0;

        if let Some(ref mut piece) = self.current_piece{
            let mut shift_y = 0;
            while  shift_y < piece.states[piece.current_state as usize][shift_y].len() &&
            (piece.y + shift_y < self.game_map.len()) {
                
                
                let mut shift_x = 0;
                let mut counter = 0;
                while shift_x < piece.states[piece.current_state as usize][shift_y].len() &&
                (piece.x + shift_x as isize) < self.game_map[piece.y + shift_y].len() as isize {
                    counter += 1;   
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] = piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                if shift_y + 1 <  piece.states[piece.current_state as usize][0].len(){
                    shift_y += 1;
                }
                else {
                    break;
                }
                println!("------shift_y : {}", shift_y);
            }
            to_add += self.current_level;
        }
        self.update_score(to_add);
        self.check_lines();
        self.current_piece = None;
    }

    fn update_score(&mut self, to_add: u32){
        self.score += to_add;
    }

    fn increase_line(&mut self){
        self.nb_lines += 1;
        if self.nb_lines > LEVEL_LINES[self.current_level as usize - 1]{
            self.current_level += 1;
        }
    }
}


fn update_vec(v: &mut Vec<u32>, value: u32 ) -> bool {
    if v.len() < NB_HIGHSCORES {
        v.push(value);
        v.sort();
        true
    }
    else{
        for entry in v.iter_mut(){
            if value > *entry {
                *entry = value;
                return true;
            }
        }
        false
    }
}

fn is_time_over(tetris: &Tetris, timer: &SystemTime) -> bool{
    match timer.elapsed(){
        Ok(elapsed) => {
            let millis = elapsed.as_secs() * 1000 + (elapsed.subsec_nanos()/1_000_000) as u64;
            millis > LEVEL_TIMES[tetris.current_level as usize - 1] as u64
        }
        Err(_) => false,
    }
}


fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime,
                                                    event_pump: &mut sdl2::EventPump) -> bool {
    let mut make_permanent = false;
    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y as isize;
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => 
                {
                    *quit = true;
                    break;
                }
                Event::KeyDown {  keycode: Some(Keycode::Down), ..} =>
                {
                    *timer = SystemTime::now();
                    tmp_y += 1;
                }
                Event::KeyDown {  keycode: Some(Keycode::Right), ..} =>
                {
                    tmp_x += 1;
                }
                Event::KeyDown {  keycode: Some(Keycode::Left), ..} =>
                {
                    tmp_x -= 1;
                }
                Event::KeyDown {  keycode: Some(Keycode::Up), ..} =>
                {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown {  keycode: Some(Keycode::Space), ..} =>
                {
                    let x = piece.x;
                    let mut y = piece.y;
                    while piece.change_position(&tetris.game_map, x, y as isize + 1) == true {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
        }
        if !make_permanent { 
            if piece.change_position(&tetris.game_map, tmp_x, tmp_y) == false &&
            tmp_y != piece.y as isize {
                make_permanent = true;
            }
        }
    }
    if make_permanent {
        tetris.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}


fn print_game_information(tetris: &Tetris){
    let mut new_highest_highscore = true;
    let mut new_highest_lines_sent = true;
    if let Some((mut highscore, mut lines_sent)) = load_highscores_and_lines(){
        new_highest_highscore = update_vec(& mut highscore, tetris.score);
        new_highest_lines_sent = update_vec(& mut lines_sent, tetris.nb_lines);
        if new_highest_highscore || new_highest_lines_sent {
            save_highscores_and_lines(&highscore, &lines_sent);
        }
    }
    else {
        save_highscores_and_lines(&[tetris.score], &[tetris.nb_lines]);
    }
    println!("Game over ...");
    println!("Score :           {}{}",
            tetris.score,
            if new_highest_highscore {
                    " [NEW HIGHSCORE]"
                } else { "" });
    println!("Number of lines: {}{}", 
                tetris.nb_lines,
            if new_highest_lines_sent { 
                " [NEW HIGHSCORE]"
            } else { "" });
    println!("Current level :   {}", tetris.current_level);
    //Check highscores here and update if needed
}


fn main() {
    let sdl_context = sdl2::init().expect("SDL Initialisation failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
    sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("Could not initialize image context");
    let width: u32 = 600;
    let height: u32 = 800;
    let window = video_subsystem.window("rust-sdl2 demo: Video", width, height)
    .position_centered()
    .opengl()
    .build()
    .expect("Failed to create window");

    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();

    let grid_x = (width - TETRIS_HEIGHT as u32 * 10) as i32 / 2;
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 /2;

    let mut canvas = window.into_canvas().build().expect("Failed to convert window into canvas");
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    // let green_square: Texture = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Green, TEXTURE_SIZE).expect("Failed to create green square texture");
    // let blue_square = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TEXTURE_SIZE).expect("Failed to create blue square texture");
    // let image_textture = texture_creator.load_texture("assets/test.jpg").expect("Couldn't load image.");
    let grid = create_texture_rect(&mut canvas, 
                        &texture_creator, 
                        TextureColor::Black, 
                        TETRIS_HEIGHT as u32 *10,
                        TETRIS_HEIGHT as u32 *16).expect("Failed to create texture");
    
    let border = create_texture_rect(&mut canvas, 
                            &texture_creator, 
                            TextureColor::White, 
                            TETRIS_HEIGHT as u32 *10 + 20,
                            TETRIS_HEIGHT as u32 *16 + 20).expect("Failed to create texture");

    macro_rules! texture {
        ($color: expr) => {
            create_texture_rect(&mut canvas,
            &texture_creator,
            $color,
            TETRIS_HEIGHT as u32,
            TETRIS_HEIGHT as u32).unwrap()
        };
    }

    let textures = [texture!(TextureColor::Blue), texture!(TextureColor::Yellow), texture!(TextureColor::Red),
    texture!(TextureColor::Brown), texture!(TextureColor::Yellow), texture!(TextureColor::Blue), texture!(TextureColor::Red)];
    
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    
    loop{
        if match timer.elapsed(){
            Ok(elapsed) => elapsed.as_secs() >= 1,
            Err(_) => false,
        }{
            if is_time_over(&tetris, &timer){
                let mut make_permanent = false;
                if let Some(ref mut piece) = tetris.current_piece{
                    let x = piece.x;
                    let y = piece.y + 1;
                    make_permanent = piece.change_position(&tetris.game_map, x, y as isize);
                }
                if make_permanent{
                    tetris.make_permanent();
                }
                timer = SystemTime::now();
            }
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.clear();
            canvas.copy(&border, None, 
                Rect::new((width - TETRIS_HEIGHT as u32 * 10) as i32/2 - 10, 
                (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2 - 10, 
                TETRIS_HEIGHT as u32 * 10 + 20, 
                TETRIS_HEIGHT as u32 * 16 +20)).expect("Couldn't copy texture into window");
            canvas.copy(&grid, None, 
                Rect::new((width - TETRIS_HEIGHT as u32 * 10) as i32/2, 
                (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2, 
                TETRIS_HEIGHT as u32 * 10, 
                TETRIS_HEIGHT as u32 * 16)).expect("Couldn't copy texture into window");
        }

        //draw tetris grid here

        if tetris.current_piece.is_none() {
            let current_piece = create_new_tetrimino();
            if !current_piece.test_current_position(&tetris.game_map){
                println!("Current piece is none");
                print_game_information(&tetris);
                //should uncomment line bellow, so game should break if current_pice is None
                // break;
            }
            tetris.current_piece = Some(current_piece);
        }
        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump){
            if let Some(ref mut piece) = tetris.current_piece{
                //we need to draw current tetrimno
                for (line_nb, line) in piece.states[piece.current_state as usize].iter().enumerate(){
                    for (case_nb, case) in line.iter().enumerate(){
                        if *case == 0{
                            continue
                        }
                        //the new part is here
                        canvas.copy(&textures[*case as usize - 1],
                             None,
                              Rect::new(grid_x + (piece.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                            grid_y + (piece.y + line_nb) as i32 * TETRIS_HEIGHT as i32, TETRIS_HEIGHT as u32, TETRIS_HEIGHT as u32)).expect("Couldn't copy texur in window");
                    }
                }
            }
        }
        if quit {
            println!("QUIT BREAK");
            print_game_information(&tetris);
            break;
        }

        for (line_nb, line) in tetris.game_map.iter().enumerate(){
            for (case_nb, case) in line.iter().enumerate(){
                if *case == 0{
                    continue
                }
                canvas.copy(&textures[*case as usize - 1], 
                    None, 
                    Rect::new(grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                    grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                TETRIS_HEIGHT as u32, TETRIS_HEIGHT as u32)).expect("Couldn't copy texur in window");
            }
        }
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
texture_creator: &'a TextureCreator<WindowContext>,
color: TextureColor, width : u32, height: u32) -> Option<Texture<'a>>{
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, width, height){
        _ = canvas.with_texture_canvas(&mut square_texture, |texture|{
            match color {
                TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255,0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0,255)),
                TextureColor::Black => texture.set_draw_color(Color::RGB(0, 0,0)),
                TextureColor::White => texture.set_draw_color(Color::RGB(255, 255,255)),
                TextureColor::Yellow => texture.set_draw_color(Color::RGB(255, 255,0)),
                TextureColor::Red => texture.set_draw_color(Color::RGB(255, 0,0)),
                TextureColor::Brown => texture.set_draw_color(Color::RGB(150, 75,0)),
            }
            texture.clear();
        }).expect("Failed to color texture");
        Some(square_texture)
    }
    else {
        None
    }
} 