use crate::{system::Cardputer, tiles};
use embedded_graphics::{pixelcolor::{Rgb565, RgbColor}, draw_target::DrawTarget, primitives::Rectangle, geometry::{Point, Size, Dimensions}, Pixel};
use embedded_graphics_framebuf::FrameBuf;
use esp_println::println;
use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use softsynth::{Sound,mix, MAX_VOL, Oscillator, Adsr};
mod music;

use hal::prelude::_embedded_hal_blocking_delay_DelayMs;

#[derive(Copy,Clone,Debug, PartialEq)]
enum Threat{
    Basic, Fast, Rubbish
}
impl Threat {
    fn color (&self) -> Rgb565 {
        match *self {
            Threat::Basic => Rgb565::new(15,30,15),
            Threat::Rubbish => Rgb565::new(8,16,8),
            Threat::Fast => Rgb565::new(31,10,10),
        }
    }
    fn tile (&self) -> usize {
        match *self {
            Threat::Basic => 148,
            Threat::Rubbish => 185,
            Threat::Fast => 256,            
        }
    }
}
struct Robots<'a> {
    player: (usize,usize),
    threats: ConstGenericRingBuffer<(usize,usize,Threat),80>,
    animations: ConstGenericRingBuffer<((i32,i32),(i32,i32),(usize,usize),Option<Threat>),81>,
    explosions: ConstGenericRingBuffer<(usize,usize),42>,
    emps: ConstGenericRingBuffer<(usize,usize),9>,
    dalek_fb: FrameBuf<Rgb565,&'a mut [Rgb565;10*10]>,
    robot_fb: FrameBuf<Rgb565,&'a mut [Rgb565;10*10]>,
    player_fb: FrameBuf<Rgb565,&'a mut [Rgb565;10*10]>,
    rubbish_fb: FrameBuf<Rgb565,&'a mut [Rgb565;10*10]>,    
    explosion_fb : FrameBuf<Rgb565,&'a mut [Rgb565;10*10]>,    
    emp_fb : FrameBuf<Rgb565,&'a mut [Rgb565;10*10]>,    
    ticks: i32,
    num_emps: usize,
    num_safe_teles: usize,
    dead:bool,
    won_level:bool,
    difficulty: Difficulty,
    level: usize,    
}
const WIDTH : usize = 30;
const HEIGHT : usize = 16;
static mut DALEK : [Rgb565;10*10] = [Rgb565::MAGENTA;100];
static mut ROBOT : [Rgb565;10*10] = [Rgb565::MAGENTA;100];
static mut RUBBISH : [Rgb565;10*10] = [Rgb565::MAGENTA;100];
static mut EXPLOSION : [Rgb565;10*10] = [Rgb565::MAGENTA;100];
static mut EMP : [Rgb565;10*10] = [Rgb565::MAGENTA;100];
static mut PLAYER : [Rgb565;10*10] = [Rgb565::MAGENTA;100];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Difficulty {
    level_increment: usize,
    danger_start: usize,
    danger_multiplier: usize,
    danger_divider: usize,
    teleports_divider: usize,
    emps_divider: usize,
    max_safe_teles: usize,
    max_emp_pulses: usize,
    starting_emps: usize,
    starting_teles: usize,
}
const EASY : Difficulty = Difficulty {
    level_increment: 2,
    danger_start: 8,
    danger_multiplier: 1,
    danger_divider: 2,
    teleports_divider: 3,
    emps_divider: 2,
    max_safe_teles: 15,
    max_emp_pulses: 20,
    starting_emps: 7,
    starting_teles: 5
};
const MEDIUM : Difficulty = Difficulty {
    level_increment: 3,
    danger_start: 4,
    danger_multiplier: 1,
    danger_divider: 1,
    teleports_divider: 5,
    emps_divider: 3,
    max_safe_teles: 10,
    max_emp_pulses: 15,
    starting_emps:5,
    starting_teles: 2,
};
const HARD : Difficulty = Difficulty {
    level_increment: 4,
    danger_start: 2,
    danger_multiplier: 2,
    danger_divider: 1,
    teleports_divider: 8,
    emps_divider: 5,
    max_safe_teles: 6,
    max_emp_pulses: 10,
    starting_emps: 1,
    starting_teles: 0
};
const TITLE : Difficulty = Difficulty {
    level_increment: 20,
    danger_start: 0,
    danger_multiplier: 2,
    danger_divider: 1,
    teleports_divider: 8,
    emps_divider: 5,
    max_safe_teles: 6,
    max_emp_pulses: 10,
    starting_emps: 1,
    starting_teles: 0
};

impl <'a> Robots<'a> {
    fn new(difficulty : Difficulty, sys : &mut Cardputer) -> Self {
        let mut dalek = unsafe { FrameBuf::new(&mut DALEK,10,10)};
        let mut robot= unsafe { FrameBuf::new(&mut ROBOT,10,10)};
        let mut rubbish= unsafe { FrameBuf::new(&mut RUBBISH,10,10)};
        let mut explosion = unsafe { FrameBuf::new(&mut EXPLOSION,10,10)};
        let mut emp = unsafe { FrameBuf::new(&mut EMP,10,10)};
        let mut player = unsafe { FrameBuf::new(&mut PLAYER,10,10)};
        tiles::draw(149,0,1+8,Some(Rgb565::YELLOW), None,  &mut explosion);
        tiles::draw(149,2,1+8,Some(Rgb565::YELLOW), None,  &mut explosion);
        tiles::draw(149,0,0+8,Some(Rgb565::YELLOW), None,  &mut explosion);
        tiles::draw(149,2,0+8,Some(Rgb565::YELLOW), None,  &mut explosion);
        tiles::draw(149,0,2+8,Some(Rgb565::YELLOW), None,  &mut explosion);
        tiles::draw(149,2,2+8,Some(Rgb565::YELLOW), None,  &mut explosion);
        tiles::draw(149,1,0+8,Some(Rgb565::YELLOW), None,  &mut explosion);
        tiles::draw(149,1,2+8,Some(Rgb565::YELLOW), None,  &mut explosion);
        tiles::draw(149,1,1+8,Some(Rgb565::RED), None,  &mut explosion);
        tiles::draw(149,0,1+8,Some(Rgb565::BLUE), None,  &mut emp);
        tiles::draw(149,2,1+8,Some(Rgb565::BLUE), None,  &mut emp);
        tiles::draw(149,0,0+8,Some(Rgb565::BLUE), None,  &mut emp);
        tiles::draw(149,2,0+8,Some(Rgb565::BLUE), None,  &mut emp);
        tiles::draw(149,0,2+8,Some(Rgb565::BLUE), None,  &mut emp);
        tiles::draw(149,2,2+8,Some(Rgb565::BLUE), None,  &mut emp);
        tiles::draw(149,1,0+8,Some(Rgb565::BLUE), None,  &mut emp);
        tiles::draw(149,1,2+8,Some(Rgb565::BLUE), None,  &mut emp);
        tiles::draw(149,1,1+8,Some(Rgb565::GREEN), None,  &mut emp);
        tiles::draw(Threat::Basic.tile(),0,1+8,Some(Rgb565::BLACK), None,  &mut dalek);
        tiles::draw(Threat::Basic.tile(),2,1+8,Some(Rgb565::BLACK), None,  &mut dalek);
        tiles::draw(Threat::Basic.tile(),0,0+8,Some(Rgb565::BLACK), None,  &mut dalek);
        tiles::draw(Threat::Basic.tile(),2,0+8,Some(Rgb565::BLACK), None,  &mut dalek);
        tiles::draw(Threat::Basic.tile(),0,2+8,Some(Rgb565::BLACK), None,  &mut dalek);
        tiles::draw(Threat::Basic.tile(),2,2+8,Some(Rgb565::BLACK), None,  &mut dalek);
        tiles::draw(Threat::Basic.tile(),1,0+8,Some(Rgb565::BLACK), None,  &mut dalek);
        tiles::draw(Threat::Basic.tile(),1,2+8,Some(Rgb565::BLACK), None,  &mut dalek);
        tiles::draw(Threat::Basic.tile(),1,1+8,Some(Threat::Basic.color()), None,  &mut dalek);
        tiles::draw(Threat::Fast.tile(),0,1+8,Some(Rgb565::BLACK), None,  &mut robot);
        tiles::draw(Threat::Fast.tile(),2,1+8,Some(Rgb565::BLACK), None,  &mut robot);
        tiles::draw(Threat::Fast.tile(),0,0+8,Some(Rgb565::BLACK), None,  &mut robot);
        tiles::draw(Threat::Fast.tile(),2,0+8,Some(Rgb565::BLACK), None,  &mut robot);
        tiles::draw(Threat::Fast.tile(),0,2+8,Some(Rgb565::BLACK), None,  &mut robot);
        tiles::draw(Threat::Fast.tile(),2,2+8,Some(Rgb565::BLACK), None,  &mut robot);
        tiles::draw(Threat::Fast.tile(),1,0+8,Some(Rgb565::BLACK), None,  &mut robot);
        tiles::draw(Threat::Fast.tile(),1,2+8,Some(Rgb565::BLACK), None,  &mut robot);
        tiles::draw(Threat::Fast.tile(),1,1+8,Some(Threat::Fast.color()), None,  &mut robot);
        tiles::draw(Threat::Rubbish.tile(),0,1+8,Some(Rgb565::BLACK), None,  &mut rubbish);
        tiles::draw(Threat::Rubbish.tile(),2,1+8,Some(Rgb565::BLACK), None,  &mut rubbish);
        tiles::draw(Threat::Rubbish.tile(),0,0+8,Some(Rgb565::BLACK), None,  &mut rubbish);
        tiles::draw(Threat::Rubbish.tile(),2,0+8,Some(Rgb565::BLACK), None,  &mut rubbish);
        tiles::draw(Threat::Rubbish.tile(),0,2+8,Some(Rgb565::BLACK), None,  &mut rubbish);
        tiles::draw(Threat::Rubbish.tile(),2,2+8,Some(Rgb565::BLACK), None,  &mut rubbish);
        tiles::draw(Threat::Rubbish.tile(),1,0+8,Some(Rgb565::BLACK), None,  &mut rubbish);
        tiles::draw(Threat::Rubbish.tile(),1,2+8,Some(Rgb565::BLACK), None,  &mut rubbish);
        tiles::draw(Threat::Rubbish.tile(),1,1+8,Some(Threat::Rubbish.color()), None,  &mut rubbish);
        tiles::draw(73,0,1+8,Some(Rgb565::BLACK), None,  &mut player);
        tiles::draw(73,2,1+8,Some(Rgb565::BLACK), None,  &mut player);
        tiles::draw(73,0,0+8,Some(Rgb565::BLACK), None,  &mut player);
        tiles::draw(73,2,0+8,Some(Rgb565::BLACK), None,  &mut player);
        tiles::draw(73,0,2+8,Some(Rgb565::BLACK), None,  &mut player);
        tiles::draw(73,2,2+8,Some(Rgb565::BLACK), None,  &mut player);
        tiles::draw(73,1,0+8,Some(Rgb565::BLACK), None,  &mut player);
        tiles::draw(73,1,2+8,Some(Rgb565::BLACK), None,  &mut player);
        tiles::draw(73,1,1+8,Some(Rgb565::YELLOW), None,  &mut player);
        /* 
         */
        let mut game = Self {
            player: (15,8),
            threats : ConstGenericRingBuffer::new(),
            dalek_fb:dalek,player_fb:player,robot_fb:robot,rubbish_fb:rubbish,explosion_fb:explosion,emp_fb:emp,
            num_safe_teles: difficulty.starting_teles, num_emps: difficulty.starting_emps,
            animations: ConstGenericRingBuffer::new(), ticks: 5, dead:false, won_level: false, difficulty,
            explosions: ConstGenericRingBuffer::new(), emps: ConstGenericRingBuffer::new(), level: 1
        };
        game.set_up_level(1,sys);
        game
    }

    fn set_up_level(&mut self, level : usize, sys : &mut Cardputer) {
        self.won_level = false;
        self.dead = false;
        self.ticks = 5;
        self.animations.clear();
        self.level = level;
        self.threats.clear();
        self.player = (WIDTH/2,HEIGHT/2);
        for _i in 0..1+level*self.difficulty.level_increment {
            let mut x;
            let mut y; 
            'outer: loop {
                x = sys.rng.random() as usize % WIDTH;                
                y = sys.rng.random() as usize % HEIGHT;                
                for (tx,ty,_) in &self.threats {                    
                    if *tx == x && *ty == y || x == 15 && y == 8 {
                        continue 'outer;
                        
                    }
                }
                break;
            }
            self.threats.push((x,y,Threat::Basic));
        }
        if level > self.difficulty.danger_start {
            for _i in 0..(level-self.difficulty.danger_start)*self.difficulty.danger_multiplier/self.difficulty.danger_divider {
                let mut x;
                let mut y; 
                'outer: loop {
                    x = sys.rng.random() as usize % WIDTH;
                    y = sys.rng.random() as usize % HEIGHT;
                    for (tx,ty,_) in &self.threats {                        
                        if *tx == x && *ty == y || x == 15 && y == 8 {                            
                            continue 'outer;
                        }
                    }
                    break;
                }
                self.threats.push((x,y,Threat::Fast));
            }            
        }
        self.num_safe_teles += level / self.difficulty.teleports_divider;
        self.num_emps += level / self.difficulty.emps_divider;
        if self.num_safe_teles > self.difficulty.max_safe_teles { self.num_safe_teles = self.difficulty.max_safe_teles };
        if self.num_emps > self.difficulty.max_emp_pulses { self.num_emps = self.difficulty.max_emp_pulses };          
    }


    fn draw(&mut self, title:bool, sys : &mut Cardputer) {
        let x_offset = 0;
        let y_offset = 8;        
        let _ = sys.display.fb.fill_solid(&sys.display.fb.bounding_box(), Rgb565::new(22,52,22));
        for x in 0..30 {
            for y in 0..16 {
                if x % 2 == y % 2 {
                     let _ = sys.display.fb.fill_solid(&Rectangle::new(Point::new(x as i32 * 8, y as i32 * 8),Size::new(8,8)), Rgb565::new(20,50,20));
                }
            }
        }
        let bg = Rgb565::new(5,20,5);
        let _ = sys.display.fb.fill_solid(&Rectangle::new(Point::new(0,135-8),Size::new(240,8)), bg);
        if !title {
            tiles::draw_num(self.num_emps, tiles::Alignment::Left, 0, 135, Some(Rgb565::GREEN), Some(bg),&mut sys.display.fb);
            tiles::draw_num(self.level, tiles::Alignment::Center, 120, 135, Some(Rgb565::WHITE), Some(bg),&mut sys.display.fb);
            tiles::draw_num(self.num_safe_teles, tiles::Alignment::Right, 240, 135, Some(Rgb565::BLUE), Some(bg),&mut sys.display.fb);
        }
        if self.ticks >= 5 {
            for (x,y,t) in &self.threats { 
                self.draw_object(Some(*t),*x as i32 * 8 + x_offset,*y as i32 * 8  - 8 + y_offset,sys);
            }
            
            if title {
                tiles::draw_window(tiles::WINDOW_DECORATED, 120-36-8-16, 4*16+y_offset-4, 15, 3, Some(bg), None, &mut sys.display.fb);
                tiles::draw_text(b"DALEKS", 120-36-16+28, 4*16 + y_offset - 4+8, Some(Rgb565::YELLOW), Some(bg),&mut sys.display.fb);
                tiles::draw_window(tiles::WINDOW_DECORATED, 120-36-8-16-16, 4*16+y_offset-4-48, 19, 4, Some(Rgb565::WHITE), None, &mut sys.display.fb);
                tiles::draw_text(b"Select Difficulty", 120-36-16-16, 4*16 + y_offset - 4-32, Some(Rgb565::BLACK), Some(Rgb565::WHITE),&mut sys.display.fb);
                tiles::draw_text(b"(1-3)", 120-20, 4*16 + y_offset - 4-40, Some(bg), Some(Rgb565::WHITE),&mut sys.display.fb);
            } else {
                self.draw_object(None,self.player.0 as i32 * 8 + x_offset,self.player.1 as i32 * 8  - 8 + y_offset,sys);
            }
            if self.dead {
                tiles::draw_window(tiles::WINDOW_DECORATED, 120-36-8, 4*16+y_offset-4-8, 11, 3, Some(Rgb565::BLACK), None, &mut sys.display.fb);
                tiles::draw_text(b"GAME OVER", 120-36, 4*16 + y_offset - 4, Some(Rgb565::RED), Some(Rgb565::BLACK),&mut sys.display.fb);
            } else if self.won_level {
                tiles::draw_window(tiles::WINDOW_DECORATED, 120-36-8-8, 4*16+y_offset-4-8, 13, 3, Some(bg), None, &mut sys.display.fb);
                tiles::draw_text(b"LEVEL CLEAR", 120-36-8, 4*16 + y_offset - 4, Some(Rgb565::GREEN), Some(bg),&mut sys.display.fb);
            }
            self.ticks = 6;
        } else {
            for (p,dp,_,t) in &self.animations {
                self.draw_object(*t, p.0 + (dp.0 * self.ticks * 2), p.1 + (dp.1 * self.ticks * 2), sys)
            }
            for p in &self.explosions {
                self.draw_explosion(false,p.0 as i32 * 8, p.1 as i32 * 8, sys);
            }
            for p in &self.emps {
                self.draw_explosion(true,p.0 as i32 * 8, p.1 as i32 * 8, sys);
            }
            self.ticks += 1;
            if self.ticks == 5 {
                self.threats.clear();
                self.won_level = true;
                for (x,y) in &self.explosions {
                    self.threats.push((*x,*y,Threat::Rubbish));
                }
                for (_,_,fp,t) in &self.animations {
                    if let Some(t) = *t { 
                        if !self.explosions.contains(fp) {
                            self.threats.push((fp.0,fp.1,t));
                            if t != Threat::Rubbish { self.won_level = false; }
                        }
                    }
                }
                self.explosions.clear();
                self.emps.clear();
                self.animations.clear();
            }
        }
    }
    fn draw_explosion(&self, emp: bool, x : i32, y : i32, sys : &mut Cardputer) {
        sys.display.fb.draw_iter(if emp { &self.emp_fb } else { &self.explosion_fb}.into_iter().filter(|x|x.1 != Rgb565::MAGENTA).map(|v| Pixel(Point::new(v.0.x + x, v.0.y + y),v.1))).unwrap();
    }
    fn draw_object(&self, object: Option<Threat>, x : i32, y : i32, sys : &mut Cardputer) {
        sys.display.fb.draw_iter(match object {
            Some(Threat::Basic) => &self.dalek_fb,
            Some(Threat::Rubbish) => &self.rubbish_fb,
            Some(Threat::Fast) => &self.robot_fb,
            None => &self.player_fb,
        }.into_iter().filter(|x|x.1 != Rgb565::MAGENTA).map(|v| Pixel(Point::new(v.0.x + x, v.0.y + y),v.1))).unwrap();
    }

    fn make_move(&mut self, mov: (i32,i32), move_opponents: bool) -> bool {
        if self.player.0 as i32 + mov.0 < 0 || self.player.1 as i32 + mov.1 < 0 { return false; }
        if self.player.1 as i32 + mov.0 >= WIDTH as i32 || self.player.1 as i32 + mov.1 >= HEIGHT as i32 { return false; }
        
        let x = (self.player.0 as i32 + mov.0) as usize;
        let y = (self.player.1 as i32 + mov.1) as usize;

        self.animations.push(((self.player.0 as i32 * 8, self.player.1 as i32 * 8),mov,(x,y),None));
        self.player=(x,y);                
        self.queue_opponent_moves(move_opponents)
    }

    fn queue_opponent_moves(&mut self, move_opponents: bool) -> bool {
        self.ticks = 0;
        let mut collisions = [[false;HEIGHT];WIDTH];
        for (x,y) in &self.emps {
            collisions[*x][*y] = true;
        }   
        let mut explosion = false;
        for (x,y,t) in &self.threats {            
            let m = if self.emps.contains(&(*x,*y)) {
                0
            } else if move_opponents { match *t {
                Threat::Basic => 1,
                Threat::Rubbish => 0,
                Threat::Fast => 2,
            }} else { 0 };
            let dp =  ((self.player.0 as i32 - *x as i32).min(m).max(-m),(self.player.1 as i32 - *y as i32).min(m).max(-m));
            let fp = ((*x as i32 + dp.0) as usize,(*y as i32 + dp.1) as usize);
            let dp1 =  ((self.player.0 as i32 - *x as i32).min(1).max(-1),(self.player.1 as i32 - *y as i32).min(1).max(-1));
            let fp1 = ((*x as i32 + dp1.0) as usize,(*y as i32 + dp1.1) as usize);
            if *t == Threat::Fast && collisions[fp1.0][fp1.1] {
                if !self.explosions.contains(&fp1) { self.explosions.push(fp1) };
                explosion = true;
                self.animations.push(((*x as i32 * 8, *y as i32 * 8),dp1,fp1,Some(*t)));
                continue;
            }
            if collisions[fp.0][fp.1] { 
                if !self.explosions.contains(&fp) { self.explosions.push(fp) };
                explosion = true;
            }  else {
                collisions[fp.0][fp.1] = true;
            }
            self.animations.push(((*x as i32 * 8, *y as i32 * 8),dp,fp,Some(*t)));
        }
        if collisions[self.player.0][self.player.1] {
            self.dead = true;
        }
        return explosion;
    }
    fn next_level(&mut self, sys : &mut Cardputer) {
        self.set_up_level(self.level + 1, sys)
    }
    fn emp(&mut self) -> bool {
        if self.num_emps == 0 { return false; }
        self.num_emps -= 1;
        for i in -1..=1 {
            for j in -1..=1 {
                let xx = self.player.0 as i32 + i;
                let yy = self.player.1 as i32 + j;
                if xx < 0 || yy < 0 || xx >= WIDTH as i32 || yy >= HEIGHT as i32 { continue }
                let x = xx as usize;
                let y = yy as usize;
                if self.player == (x,y) { continue }
                self.emps.push((x,y));                
            }
        }
        self.queue_opponent_moves(true);
        return true;
    }
    fn teleport(&mut self, safe: bool, sys : &mut Cardputer) -> bool {
        if safe && self.num_safe_teles == 0 { return false; }
        if safe {
            self.num_safe_teles -= 1;
        }
        let mut xx = (sys.rng.random() % WIDTH as u32) as usize;
        let mut yy = (sys.rng.random() % HEIGHT as u32) as usize;
        while safe && self.threats.iter().filter(|(x,y,_)| *x == xx && *y == yy ).next() != None {
            xx = (sys.rng.random() % WIDTH as u32) as usize;
            yy = (sys.rng.random() % HEIGHT as u32) as usize;
        }
         
        let dx = xx as i32 - self.player.0 as i32;
        let dy = yy as i32 - self.player.1 as i32;
        self.make_move((dx,dy), !safe);
        return true;
    }
}

pub fn robots(sys : &mut Cardputer) {
    let bass_osc = music::bass_osc();    
    let treble_osc = music::treble_osc();
    let mut game = Robots::new(TITLE,sys);
    let mut v = mix(bass_osc.clone().into_player(&music::EMPTY_SONG),treble_osc.clone().into_player(&music::EMPTY_SONG));
    let mut bass_index = 0;
    let mut treble_index = 0;
    let mut sfx_osc = Adsr::new(Oscillator::noise(), 1, 300, MAX_VOL / 3 * 2, 10);
    let mut sfx_os2 = Adsr::new(Oscillator::default(), 1, 300, MAX_VOL / 3 * 2, 10);
    sfx_osc.set_vol(MAX_VOL/3);
    sfx_os2.set_vol(MAX_VOL/3);
    let mut title_screen = true;
    let mut wrap_up_music = false;
    let mut sfx = sfx_osc.clone().into_player(&music::EMPTY_SONG);
    loop {
        if v.len() < 100 {
            if wrap_up_music {
                v = mix(bass_osc.clone().into_player(&music::BASS_END)
                       , treble_osc.clone().into_player(&music::EMPTY_SONG));
                bass_index = 0;
                wrap_up_music = false;
                treble_index = 0;
            } else if !title_screen {
                v = mix(bass_osc.clone().into_player(music::BASS_PATCHES[bass_index])
                       , treble_osc.clone().into_player(music::TREBLE_PATCHES[treble_index]));
                bass_index = (bass_index + 1) % music::BASS_PATCHES.len();
                treble_index = (treble_index + 1) % music::TREBLE_PATCHES.len();
            }
        }

        if sys.button_pressed() {
            return;   
        }
        let b = sys.keyboard.is_change();
        if b && !sys.keyboard.keys_state.word.is_empty() && game.ticks > 5 {
            if title_screen {
                match sys.keyboard.keys_state.word[0] {
                    b'1' => { game = Robots::new(EASY,sys); title_screen = false},
                    b'2' => {game = Robots::new(MEDIUM,sys); title_screen = false},
                    b'3' => {game = Robots::new(HARD,sys); title_screen = false},
                    _ => {},
                }
            } else if game.dead {
                title_screen = true;
                wrap_up_music = true;
                game = Robots::new(TITLE, sys);
            } else if game.won_level {
                game.next_level(sys);
            } else {
                match sys.keyboard.keys_state.word[0] {
                    b'[' => if game.make_move((0,1),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b'p'  => if game.make_move((-1,1),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b']'  => if game.make_move((1,1),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b'l'  => if game.make_move((-1,0),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b';'  => if game.make_move((0,0),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b'\'' => if game.make_move((1,0),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b',' => if game.make_move((-1,-1),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b'.' => if game.make_move((0,-1),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b'/' => if game.make_move((1,-1),true) {
                        sfx = sfx_osc.clone().into_player(&music::EXP_SFX);
                    },
                    b't' => if game.teleport(false, sys) {
                        sfx = sfx_os2.clone().into_player(&music::TEL_SFX);
                    } else {
                        sfx = sfx_osc.clone().into_player(&music::DUD_SFX);
                    },
                    b'r' => if game.teleport(true, sys) {
                        sfx = sfx_os2.clone().into_player(&music::TEL_SFX);
                    } else {
                        sfx = sfx_osc.clone().into_player(&music::DUD_SFX);
                    },
                    b'e' => if game.emp() {
                        sfx = sfx_osc.clone().into_player(&music::EMP_SFX);
                    } else {
                        sfx = sfx_osc.clone().into_player(&music::DUD_SFX);
                    },
                    _ => {},
                }
                if game.dead {
                    sfx = sfx_osc.clone().into_player(&music::DED_SFX);
                }             }
        }
        if sys.tick(&mut mix(&mut v,&mut sfx)) && game.ticks <= 5 {
            game.draw(title_screen, sys);
            if !game.dead && game.won_level {
                sfx = sfx_os2.clone().into_player(&music::WON_SFX);  
            }

        }
        sys.delay.delay_ms(1u32);
    }
}