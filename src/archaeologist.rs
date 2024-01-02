use embedded_graphics::{draw_target::DrawTarget, geometry::Dimensions};
use esp_backtrace as _;
use hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use ringbuffer::RingBuffer;
use softsynth::{songs::Score, mix};
use crate::system::Cardputer;
use crate::tiles;
use embedded_graphics::{pixelcolor::Rgb565, prelude::RgbColor,prelude::Point, primitives::Rectangle};
use softsynth::{pitch::*, Oscillator, MAX_VOL, Adsr, Sound};


#[derive(Debug, Copy, Clone)]
struct Cell {
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
    neighbours: u8,    
}

const WIDTH : usize = 15;
const HEIGHT : usize =8;
struct Minesweeper {
    board: [[Cell;HEIGHT];WIDTH],
    num_flags: u8,
    total_mines: u8,
    cursor: (usize,usize),
    game_over: bool,
    has_won: bool,
}

impl Minesweeper {
    pub fn new (sys : &mut Cardputer, mut num_mines: u8) -> Minesweeper {
        let total_mines = num_mines;
        let mut board = [[Cell { is_mine: false, is_revealed: false, is_flagged: false, neighbours: 0};HEIGHT];WIDTH];
        let num_flags = num_mines;
        while num_mines > 0 { 
            let x = sys.rng.random() as usize % WIDTH;
            let y = sys.rng.random() as usize % HEIGHT;
            if board[x][y].is_mine {
                continue
            } else {
                board[x][y].is_mine = true;
                if x > 0 {
                    board[x-1][y].neighbours += 1;
                    if y > 0 { 
                        board[x-1][y-1].neighbours += 1;
                    }
                    if y < HEIGHT - 1 {
                        board[x-1][y+1].neighbours += 1;
                    }
                }
                if x < WIDTH - 1 {
                    board[x+1][y].neighbours += 1;
                    if y > 0 { 
                        board[x+1][y-1].neighbours += 1;
                    }
                    if y < HEIGHT - 1 {
                        board[x+1][y+1].neighbours += 1;
                    }
                }
                if y > 0 { 
                    board[x][y-1].neighbours += 1;
                }
                if y < HEIGHT - 1 {
                    board[x][y+1].neighbours += 1;
                }
                board[x][y].neighbours += 1;
                num_mines -= 1;
            }
        }

        Minesweeper {
            board, num_flags, total_mines,cursor: (2,2), game_over: false,has_won:false,
        }
    }
    fn move_up(&mut self) {
        if self.cursor.1 < HEIGHT - 1 {
            self.cursor.1 += 1;
        }
    }
    fn move_down(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        }
    }
    fn move_right(&mut self) {
        if self.cursor.0 < WIDTH - 1 {
            self.cursor.0 += 1;
        }
    }
    fn move_left(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
        }
    }
    fn place_flag(&mut self) {
        if !self.board[self.cursor.0][self.cursor.1].is_revealed {
            if self.board[self.cursor.0][self.cursor.1].is_flagged {
                self.board[self.cursor.0][self.cursor.1].is_flagged = false;
                self.num_flags +=1;
            } else if self.num_flags > 0 {
                self.board[self.cursor.0][self.cursor.1].is_flagged = true;
                self.num_flags -=1;
            }
        }
    }
    fn reveal(&mut self, x:usize, y:usize) -> bool {
        if self.board[x][y].is_flagged { return false };
        let mut changed = false;
        if !self.board[x][y].is_revealed { changed = true };
        self.board[x][y].is_revealed = true;
        changed
    }
    fn flood(&mut self) {
        loop {
            let mut changed = false;
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    if self.board[x][y].is_revealed && self.board[x][y].neighbours == 0 {
                        if y > 0 {
                            if x > 0 {
                                changed = changed || self.reveal(x-1, y-1);
                            }
                            if x < WIDTH - 1 {
                                changed = changed || self.reveal(x+1, y-1);
                            }
                            changed = changed || self.reveal(x, y-1);
                        }
                        if y < HEIGHT - 1 {
                            if x > 0 {
                                changed = changed || self.reveal(x-1, y+1);
                            }
                            if x < WIDTH - 1 {
                                changed = changed || self.reveal(x+1, y+1);
                            }
                            changed = changed || self.reveal(x, y+1);
                        }
                        if x > 0 {
                            changed = changed || self.reveal(x-1, y);
                        }
                        if x < WIDTH - 1 {
                            changed = changed || self.reveal(x+1, y);
                        }
                    }
                }
            }
            if !changed { break };
        }
    }
    fn dig(&mut self) {
        self.reveal(self.cursor.0, self.cursor.1);
        if self.board[self.cursor.0][self.cursor.1].is_mine {
            self.game_over();   
        } else {
            self.flood();
            self.check_won();
        }
    }
    fn check_won(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if !self.board[x][y].is_revealed && !self.board[x][y].is_mine {
                    return;
                }
            }
        }
        self.game_over = true;
        self.has_won = true;
    }
    fn game_over(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.board[x][y].is_revealed = true;
            }
        }
        self.game_over = true;
    }
    fn draw(&self, title:bool, sys : &mut Cardputer) {
        let x_offset = 3;
        let y_offset = 16;
        let bar_width = 240*(self.num_flags as u32)/(self.total_mines as u32);
        sys.display.fb.fill_solid(&Rectangle::new(Point::new(0,0),(bar_width,5).into()), Rgb565::RED).unwrap();
        sys.display.fb.fill_solid(&Rectangle::new(Point::new(bar_width as i32,0),(240-bar_width,5).into()), Rgb565::BLACK).unwrap();
                
        for x in 0..WIDTH {
            for y in 0..HEIGHT {                
                let blank = self.board[x][y].is_revealed || title && self.board[x][y].neighbours == 0;                    
                let below = y > 0 && !(self.board[x][y-1].is_revealed || title && self.board[x][y-1].neighbours == 0);
                let above = y < HEIGHT-1 && !(self.board[x][y+1].is_revealed || title && self.board[x][y+1].neighbours == 0);
                let leftw = x > 0 && !(self.board[x-1][y].is_revealed || title && self.board[x-1][y].neighbours == 0);
                let right = x < WIDTH -1 && !(self.board[x+1][y].is_revealed || title && self.board[x+1][y].neighbours == 0);
                let tl = if blank { 1 } else { match (above,leftw) {
                    (false,false) => 473,
                    (false,true) => 201,
                    (true,false) => 186,
                    (true,true) => 0,
                }};
                let tr = if blank { 1 } else { match (above,right) {
                    (false,false) => 442,
                    (false,true) => 201,
                    (true,false) => 184,
                    (true,true) => 0,
                }};
                let bl = if blank { 1 } else { match (below,leftw) {
                    (false,false) => 489,
                    (false,true) => 169,
                    (true,false) => 186,
                    (true,true) => 0,
                }};
                let br = if blank { 1 } else { match (below,right) {
                    (false,false) => 490,
                    (false,true) => 169,
                    (true,false) => 184,
                    (true,true) => 0,
                }};
                let col = Rgb565::new(200,200,200);
                
                tiles::draw(bl, x as i32 * 16 + x_offset - 4, y as i32 * 16 + y_offset - 4, if bl > 400 { Some(col) } else { Some(Rgb565::BLACK) }, if bl > 400 { Some(Rgb565::BLACK) } else { Some(col) }, &mut sys.display.fb);                    
                tiles::draw(br, x as i32 * 16 + x_offset - 4 + 8, y as i32 * 16 + y_offset - 4, if br > 400 { Some(col) } else { Some(Rgb565::BLACK) }, if br > 400 { Some(Rgb565::BLACK) } else { Some(col) }, &mut sys.display.fb);                    
                tiles::draw(tl, x as i32 * 16 + x_offset - 4, y as i32 * 16 + 8 + y_offset - 4, if tl > 400 { Some(col) } else { Some(Rgb565::BLACK) }, if tl > 400 { Some(Rgb565::BLACK) } else { Some(col) }, &mut sys.display.fb);                    
                tiles::draw(tr, x as i32 * 16 + x_offset - 4 + 8, y as i32 * 16 + 8 + y_offset - 4, if tr > 400 { Some(col) } else { Some(Rgb565::BLACK) }, if tr > 400 { Some(Rgb565::BLACK) } else { Some(col) }, &mut sys.display.fb);
            }
        }
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let c = self.board[x][y];
                if !c.is_revealed {
                    if c.is_flagged {
                        tiles::draw(112, x as i32 * 16 + x_offset, y as i32 * 16 + y_offset, Some(Rgb565::RED), None, &mut sys.display.fb)    
                    }
                } else if c.is_mine {
                    tiles::draw(149, x as i32 * 16 + x_offset, y as i32 * 16 + y_offset, Some(Rgb565::RED), None, &mut sys.display.fb)
                } else if c.neighbours > 0 {
                    let col = match c.neighbours {
                        1 => Rgb565::WHITE,
                        2 => Rgb565::YELLOW,
                        3 => Rgb565::GREEN,
                        4 => Rgb565::CYAN,
                        5 => Rgb565::BLUE,
                        6 => Rgb565::MAGENTA,
                        _ => Rgb565::RED,
                    };
                    tiles::draw(c.neighbours as usize + 51, x as i32 * 16 + x_offset, y as i32 * 16 + y_offset, Some(col), None, &mut sys.display.fb);
                }                
            }
        }
        
        let col = Rgb565::WHITE;
        if !self.game_over &&!title {
            tiles::draw(124, self.cursor.0 as i32 * 16 + x_offset - 4, self.cursor.1 as i32 * 16 + y_offset - 4, Some(col), None, &mut sys.display.fb);                    
            tiles::draw(125, self.cursor.0 as i32 * 16 + x_offset - 4 + 8, self.cursor.1 as i32 * 16 + y_offset - 4, Some(col), None, &mut sys.display.fb);                    
            tiles::draw(108, self.cursor.0 as i32 * 16 + x_offset - 4, self.cursor.1 as i32 * 16 + 8 + y_offset - 4, Some(col), None, &mut sys.display.fb);                    
            tiles::draw(109, self.cursor.0 as i32 * 16 + x_offset - 4 + 8, self.cursor.1 as i32 * 16 + 8 + y_offset - 4, Some(col), None, &mut sys.display.fb);                    
        } else if self.game_over { 
            if self.has_won {
                tiles::draw_window(tiles::WINDOW_DECORATED, 120-32-8, 4*16+y_offset-4-8, 10, 3, Some(Rgb565::YELLOW), None, &mut sys.display.fb);
                tiles::draw_text(b"YOU WIN!", 120-32, 4*16 + y_offset - 4, Some(Rgb565::BLACK), Some(Rgb565::YELLOW),&mut sys.display.fb);
            } else {
                tiles::draw_window(tiles::WINDOW_DECORATED, 120-36-8, 4*16+y_offset-4-8, 11, 3, Some(Rgb565::WHITE), None, &mut sys.display.fb);
                tiles::draw_text(b"GAME OVER", 120-36, 4*16 + y_offset - 4, Some(Rgb565::RED), Some(Rgb565::WHITE),&mut sys.display.fb);
            }
        } else if title {
            tiles::draw_window(tiles::WINDOW_DECORATED, 120-36-8-16, 4*16+y_offset-4, 15, 3, Some(Rgb565::YELLOW), None, &mut sys.display.fb);
            tiles::draw_text(b"ARCHAEOLOGIST", 120-36-16, 4*16 + y_offset - 4+8, Some(Rgb565::BLACK), Some(Rgb565::YELLOW),&mut sys.display.fb);
            tiles::draw_window(tiles::WINDOW_DECORATED, 120-36-8-16-16, 4*16+y_offset-4-48, 19, 4, Some(Rgb565::WHITE), None, &mut sys.display.fb);
            tiles::draw_text(b"Select Difficulty", 120-36-16-16, 4*16 + y_offset - 4-32, Some(Rgb565::BLACK), Some(Rgb565::WHITE),&mut sys.display.fb);
            tiles::draw_text(b"(1-9)", 120-20, 4*16 + y_offset - 4-40, Some(Rgb565::BLUE), Some(Rgb565::WHITE),&mut sys.display.fb);
        }
    }
}

const BASS : Score = Score {
    tempo: 120/4,
    notes: &[
        (D3,1,4,95),
        (E3,1,2,50),
        (E3,1,2,50),
        (F3,1,4,95),
        (E3,1,4,95),
        (D3,1,4,95),
    ]
};
const TREBLE_1 : Score = Score {
    tempo: 120/4,
    notes: &[
    (D4,1,4,95),
    (E4,1,4,95),
    (E4,1,4,95),
    (E4,1,4,95),
    (F4,1,4,95),
    (A4,1,4,95),
    (E4,1,2,50),
    ]
};
const TREBLE_2 : Score = Score {
    tempo: 120/4,
    notes: &[
    (D4,1,4,95),
    (E4,1,4,95),
    (E4,1,4,95),
    (E4,1,4,95),
    (F4,1,4,95),
    (A4,1,4,95),
    (E4,1,4,95),
    (E4,1,8,95),
    (E4,1,8,95),
    ]    
};
const TREBLE_3 : Score = Score {
    tempo: 120/4,
    notes: &[
    (B4,1,4,95),
    (AF4,1,8,95),
    (AF4,1,8,95),
    (A4,1,4,95),
    (F4,1,4,95),
    (E4,1,4,95),
    (F4,1,4,95),
    (E4,1,2,50)]
};
const TREBLE_4 : Score = Score {
    tempo: 120/4,
    notes: &[
        (E4,1,4,95),
        (B4,1,4,95),
        (B4,1,4,95),
        (B4,1,4,95),
        (B4,1,8,95),
        (B4,1,8,95),
        (A4,1,4,95),
        (B4,1,2,50),
    ]
};
const TREBLE_END : Score = Score {
    tempo: 120/8,
    notes: &[
    (B4,1,4,95),
    (AF4,1,8,95),
    (AF4,1,8,95),
    (A4,1,4,95),
    (F4,1,4,95),
    (E4,1,4,95),
    (F4,1,4,95),
    (E4,1,2,50)]
};
const BASS_END : Score = Score {
    tempo: 120/8,
    notes: &[
        (D3,1,4,95),
        (D3,1,2,50),
        (D3,1,2,50),
        (D3,1,4,95),
        (E3,1,2,50),        
    ]
};
const EMPTY_SONG : Score = Score { tempo: 120/8, notes: &[]};
const TREBLE_PATCHES : &'static[&'static Score] = &[&TREBLE_1, &TREBLE_1, &TREBLE_2, &TREBLE_3,&TREBLE_1, &TREBLE_1, &TREBLE_2, &TREBLE_3, &TREBLE_4, &TREBLE_4,&TREBLE_4,&TREBLE_3];

const DIG_SFX : Score = Score {
    tempo: 120/2,
    notes: &[(A0,1,16,95)],
};
pub fn minesweeper(sys : &mut Cardputer) {
    let mut treble_index = 0;
    //let mut sequencer = Sequencer::new(&[],&[],&[],&[], false);
        
    let _ = sys.display.fb.fill_solid(&sys.display.fb.bounding_box(), Rgb565::BLACK); 
    let mut game = Minesweeper::new(sys,10);
    sys.display.backlight_on();
    let mut key_repeat : u32 = 0;
    game.draw(true,sys);
    let mut game_over_music = false;
    let mut title_screen = true;
    let mut bass_osc = Adsr::new(Oscillator::default(), 100, 300, MAX_VOL / 3 * 2, 10);
    bass_osc.set_vol(MAX_VOL/3);
    let mut treble_osc = Adsr::new(Oscillator::default(), 10, 300, MAX_VOL /8 * 2, 100);
    treble_osc.set_vol(MAX_VOL/3);
    let mut v = mix(bass_osc.clone().into_player(&EMPTY_SONG),treble_osc.clone().into_player(&EMPTY_SONG));
    let mut sfx_osc = Adsr::new(Oscillator::noise(), 1, 300, MAX_VOL / 3 * 2, 10);
    sfx_osc.set_vol(MAX_VOL/3);
    let mut sfx = sfx_osc.clone().into_player(&EMPTY_SONG);
    loop {        
        if sys.button_pressed() {
            return;
        }
        if v.len() < 100 {
            if game_over_music {
                treble_index = 0;
                v = mix(bass_osc.clone().into_player(&BASS_END),treble_osc.clone().into_player(&TREBLE_END));
                game_over_music = false;                
            } else if !title_screen && !game.game_over {
                v = mix(bass_osc.clone().into_player(&BASS),treble_osc.clone().into_player(TREBLE_PATCHES[treble_index]));
                treble_index = (treble_index + 1) % TREBLE_PATCHES.len();
            }
        }
        let b = sys.keyboard.is_change();
        if (!sys.keyboard.keys_state.word.is_empty() || sys.keyboard.keys_state.enter) && (key_repeat == 0) {
            if title_screen {
                if !sys.keyboard.keys_state.word.is_empty() {
                    title_screen = false;
                    match sys.keyboard.keys_state.word[0] {
                        b'1' => { game = Minesweeper::new(sys,4); }
                        b'2' => { game = Minesweeper::new(sys,7); }
                        b'3' => { game = Minesweeper::new(sys,10); }
                        b'4' => { game = Minesweeper::new(sys,13); }
                        b'5' => { game = Minesweeper::new(sys,16); }
                        b'6' => { game = Minesweeper::new(sys,19); }
                        b'7' => { game = Minesweeper::new(sys,22); }
                        b'8' => { game = Minesweeper::new(sys,25); }
                        b'9' => { game = Minesweeper::new(sys,28); }
                        b'0' => { game = Minesweeper::new(sys,31); }
                        _ => { title_screen = true }
                    }
                    if !title_screen {
                        //sequencer = Sequencer::new(&T1,&T2,&[],&[], true);
                    }
                }
            } else if game.game_over {
                title_screen = true;
                game = Minesweeper::new(sys, 10);
            } else if sys.keyboard.keys_state.enter {
                    game.place_flag();
            } else {
                match sys.keyboard.keys_state.word[0] {
                    b'/' => game.move_right(),
                    b',' => game.move_left(),
                    b';' => game.move_up(),
                    b'.' => game.move_down(),
                    b'f' => game.place_flag(),
                    b' ' => { game.dig(); sfx = sfx_osc.clone().into_player(&DIG_SFX);},
                    _ => {},
                }
            }
            if game.game_over && !title_screen {
                game_over_music = true;
            }
            game.draw(title_screen,sys);
            key_repeat = if b { 12} else { 2 };
        } else {
            if sys.keyboard.keys_state.word.is_empty() && !sys.keyboard.keys_state.enter {
                key_repeat = 0;
            }
        }
        
        if sys.tick(&mut mix(&mut v,&mut sfx)) && key_repeat > 0 {
            key_repeat -= 1;
        }
        sys.delay.delay_ms(1u32);
    }
}
