use embedded_graphics::{draw_target::DrawTarget,prelude::{Point, PixelColor}, Pixel};

static TILE_SET : &'static [u8;4096] = include_bytes!("../tiles");
const CHAR_MAP : [usize;256] = create_character_map();

pub const WINDOW_DECORATED : [[usize;3];3] = [[219,220,221],[235,1,237],[251,252,253]];
const fn create_character_map () -> [usize;256] {
    let mut c = [ 0 as usize; 256];
    c['A' as usize] = 2;
    c['B' as usize] = 3;
    c['C' as usize] = 4;
    c['D' as usize] = 5;
    c['E' as usize] = 6;
    c['F' as usize] = 7;
    c['G' as usize] = 8;
    c['H' as usize] = 9;
    c['I' as usize] = 10;
    c['J' as usize] = 11;
    c['K' as usize] = 12;
    c['L' as usize] = 13;
    c['M' as usize] = 14;
    c['N' as usize] = 15;
    c['O' as usize] = 16;
    c['P' as usize] = 17;
    c['Q' as usize] = 18;
    c['R' as usize] = 19;
    c['S' as usize] = 20;
    c['T' as usize] = 21;
    c['U' as usize] = 22;
    c['V' as usize] = 23;
    c['W' as usize] = 24;
    c['X' as usize] = 25;
    c['Y' as usize] = 26;
    c['Z' as usize] = 27;
    c['a' as usize] = 78;
    c['b' as usize] = 79;
    c['c' as usize] = 80;
    c['d' as usize] = 81;
    c['e' as usize] = 82;
    c['f' as usize] = 83;
    c['g' as usize] = 84;
    c['h' as usize] = 85;
    c['i' as usize] = 86;
    c['j' as usize] = 87;
    c['k' as usize] = 88;
    c['l' as usize] = 89;
    c['m' as usize] = 90;
    c['n' as usize] = 91;
    c['o' as usize] = 92;
    c['p' as usize] = 93;
    c['q' as usize] = 94;
    c['r' as usize] = 95;
    c['s' as usize] = 96;
    c['t' as usize] = 97;
    c['u' as usize] = 98;
    c['v' as usize] = 99;
    c['w' as usize] = 100;
    c['x' as usize] = 101;
    c['y' as usize] = 102;
    c['z' as usize] = 103;
    c['.' as usize] = 28;
    c['!' as usize] = 29;
    c['?' as usize] = 30;
    c['-' as usize] = 31;
    c[',' as usize] = 32;
    c['\'' as usize] = 33;
    c[':' as usize] = 34;
    c[';' as usize] = 35;
    c['_' as usize] = 36;
    c[')' as usize] = 37;
    c['(' as usize] = 38;
    c['/' as usize] = 39;
    c['\\' as usize] = 40;
    c[']' as usize] = 41;
    c['[' as usize] = 42;
    c['>' as usize] = 43;
    c['<' as usize] = 44;
    c['}' as usize] = 45;
    c['{' as usize] = 46;
    c['"' as usize] = 47;
    c['|' as usize] = 48;
    c['+' as usize] = 49;
    c['=' as usize] = 50;
    c['~' as usize] = 61;
    c['`' as usize] = 62;
    c['$' as usize] = 63;
    c['#' as usize] = 72;
    c['@' as usize] = 73;
    c['%' as usize] = 74;
    c['^' as usize] = 75;
    c['&' as usize] = 76;
    c['*' as usize] = 77;
    c['0' as usize] = 51;
    c['1' as usize] = 52;
    c['2' as usize] = 53;
    c['3' as usize] = 54;
    c['4' as usize] = 55;
    c['5' as usize] = 56;
    c['6' as usize] = 57;
    c['7' as usize] = 58;
    c['8' as usize] = 59;
    c['9' as usize] = 60;
    c
}
struct PixelIterator<C> {
    fg: Option<C>,
    bg: Option<C>,
    index: usize,
    remaining: u8,
    bits : u8,
    x : i32,
    cx : i32,
    y : i32,
}
impl <C : Copy + PixelColor> Iterator for PixelIterator<C> {
    type Item = Pixel<C>;
    fn next(&mut self) -> Option<Self::Item> {
        while self.remaining != 0 {
            if self.remaining & 0b0111 == 0 {
                self.bits = TILE_SET[self.index];
                self.index += 1;
                self.y -= 1;
                self.cx = self.x;
            }
            self.remaining -= 1;
            let coords = Point::new(self.cx,self.y);
            let b =  self.bits & 1 != 0;
            self.bits >>= 1;
            self.cx += 1;
            let c = if b { self.fg } else { self.bg };
            if let Some(x) = c {
               return Some(Pixel(coords, x));
            }
        }
        None
    }
}
pub fn index_of_char(char : u8) -> usize {
    CHAR_MAP[char as usize]
}
pub fn draw_text<B : DrawTarget>(str: &[u8], x : i32, y : i32, fg: Option<B::Color>, bg: Option<B::Color>, buf:&mut B) {
    let mut xx = x;
    for i in str {
        self::draw(self::index_of_char(*i),xx,y,fg,bg,buf);
        xx += 8;
    }
}
pub fn num_digits (mut str : usize) -> usize {
    let mut i = 0;
    while str != 0 {
        str /= 10;
        i += 1;
    }
    i.max(1)
}
pub enum Alignment { Left, Center, Right }
pub fn draw_num<B : DrawTarget>(mut str: usize, align: Alignment, x : i32, y : i32, fg: Option<B::Color>, bg: Option<B::Color>, buf:&mut B) {
    let mut xx = match align {
        Alignment::Left => x + (num_digits(str) * 8) as i32,
        Alignment::Right=> x,
        Alignment::Center =>x + (num_digits(str) * 8) as i32/2,
    };
    if str == 0 {
        self::draw(51,xx - 8,y,fg,bg,buf);
    } else {
        while str != 0 {
            xx -= 8;
            let d = str % 10;
            self::draw(51 + d,xx,y,fg,bg,buf);            
            str /= 10;
        }
    }
    
}
pub fn draw_window<B : DrawTarget>(window: [[usize;3];3], x: i32, y: i32, w: usize, h: usize, fg: Option<B::Color>, bg: Option<B::Color>, buf: &mut B) {
    draw(window[2][0], x,y,fg,bg,buf);
    for ix in 1..w-1 { 
        draw(window[2][1], x + (ix as i32) * 8,y,fg,bg,buf);
    }
    draw(window[2][2], x + (w as i32 -1) * 8, y, fg,bg,buf);    
    for iy in 1..h-1 {
        draw(window[1][0], x,y+(iy as i32)*8,fg,bg,buf);
        for ix in 1..w-1 { 
            draw(window[1][1], x + (ix as i32) * 8,y + (iy as i32) * 8,fg,bg,buf);
        }
        draw(window[1][2], x + (w as i32 -1) * 8, y+ (iy as i32) * 8, fg,bg,buf);    
    }
    draw(window[0][0], x,y+(h as i32-1)*8,fg,bg,buf);
    for ix in 1..w-1 { 
        draw(window[0][1], x + (ix as i32) * 8,y + (h as i32-1) * 8,fg,bg,buf);
    }
    draw(window[0][2], x + (w as i32 -1) * 8, y+ (h as i32-1) * 8, fg,bg,buf);    
}

pub fn draw<B : DrawTarget>(index:usize, x: i32, y : i32, fg: Option<B::Color>, bg: Option<B::Color>, buf:&mut B) {
    let i = index << 3;
    let it = PixelIterator {
        fg ,
        bg ,
        index: i,
        remaining: 64,
        bits:0,
        cx: x,
        x: x,
        y: y,
    };    
    let _ = buf.draw_iter( it);
}