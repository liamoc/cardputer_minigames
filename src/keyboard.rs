use hal::gpio::{AnyPin, Input,PullUp, Output, PushPull};
use hal::prelude::*;
use ringbuffer::{RingBuffer,ConstGenericRingBuffer};
const SHIFT : u8 = 0x80;

const KEY_LEFT_CTRL : u8 = 0x80;
const KEY_LEFT_SHIFT : u8 = 0x81;
const KEY_LEFT_ALT : u8 = 0x82;
const KEY_FN : u8 = 0xff;
const KEY_OPT : u8 = 0x00;

const KEY_BACKSPACE :u8 = 0x2a;
const KEY_TAB : u8 = 0x2b;
const KEY_ENTER : u8 = 0x28;

const KB_ASCIIMAP : [u8;128] = [
    0x00,           // NUL
    0x00,           // SOH
    0x00,           // STX
    0x00,           // ETX
    0x00,           // EOT
    0x00,           // ENQ
    0x00,           // ACK
    0x00,           // BEL
    KEY_BACKSPACE,  // BS	Backspace
    KEY_TAB,        // TAB	Tab
    KEY_ENTER,      // LF	Enter
    0x00,           // VT
    0x00,           // FF
    0x00,           // CR
    0x00,           // SO
    0x00,           // SI
    0x00,           // DEL
    0x00,           // DC1
    0x00,           // DC2
    0x00,           // DC3
    0x00,           // DC4
    0x00,           // NAK
    0x00,           // SYN
    0x00,           // ETB
    0x00,           // CAN
    0x00,           // EM
    0x00,           // SUB
    0x00,           // ESC
    0x00,           // FS
    0x00,           // GS
    0x00,           // RS
    0x00,           // US

    0x2c,          //  ' '
    0x1e | SHIFT,  // !
    0x34 | SHIFT,  // "
    0x20 | SHIFT,  // #
    0x21 | SHIFT,  // $
    0x22 | SHIFT,  // %
    0x24 | SHIFT,  // &
    0x34,          // '
    0x26 | SHIFT,  // (
    0x27 | SHIFT,  // )
    0x25 | SHIFT,  // *
    0x2e | SHIFT,  // +
    0x36,          // ,
    0x2d,          // -
    0x37,          // .
    0x38,          // /
    0x27,          // 0
    0x1e,          // 1
    0x1f,          // 2
    0x20,          // 3
    0x21,          // 4
    0x22,          // 5
    0x23,          // 6
    0x24,          // 7
    0x25,          // 8
    0x26,          // 9
    0x33 | SHIFT,  // :
    0x33,          // ;
    0x36 | SHIFT,  // <
    0x2e,          // =
    0x37 | SHIFT,  // >
    0x38 | SHIFT,  // ?
    0x1f | SHIFT,  // @
    0x04 | SHIFT,  // A
    0x05 | SHIFT,  // B
    0x06 | SHIFT,  // C
    0x07 | SHIFT,  // D
    0x08 | SHIFT,  // E
    0x09 | SHIFT,  // F
    0x0a | SHIFT,  // G
    0x0b | SHIFT,  // H
    0x0c | SHIFT,  // I
    0x0d | SHIFT,  // J
    0x0e | SHIFT,  // K
    0x0f | SHIFT,  // L
    0x10 | SHIFT,  // M
    0x11 | SHIFT,  // N
    0x12 | SHIFT,  // O
    0x13 | SHIFT,  // P
    0x14 | SHIFT,  // Q
    0x15 | SHIFT,  // R
    0x16 | SHIFT,  // S
    0x17 | SHIFT,  // T
    0x18 | SHIFT,  // U
    0x19 | SHIFT,  // V
    0x1a | SHIFT,  // W
    0x1b | SHIFT,  // X
    0x1c | SHIFT,  // Y
    0x1d | SHIFT,  // Z
    0x2f,          // [
    0x31,          // bslash
    0x30,          // ]
    0x23 | SHIFT,  // ^
    0x2d | SHIFT,  // _
    0x35,          // `
    0x04,          // a
    0x05,          // b
    0x06,          // c
    0x07,          // d
    0x08,          // e
    0x09,          // f
    0x0a,          // g
    0x0b,          // h
    0x0c,          // i
    0x0d,          // j
    0x0e,          // k
    0x0f,          // l
    0x10,          // m
    0x11,          // n
    0x12,          // o
    0x13,          // p
    0x14,          // q
    0x15,          // r
    0x16,          // s
    0x17,          // t
    0x18,          // u
    0x19,          // v
    0x1a,          // w
    0x1b,          // x
    0x1c,          // y
    0x1d,          // z
    0x2f | SHIFT,  // {
    0x31 | SHIFT,  // |
    0x30 | SHIFT,  // }
    0x35 | SHIFT,  // ~
    0              // DEL
];

struct Chart(u8, u8, u8); //value, x_1, x_2
const X_MAP_CHART : [Chart;7] = [ Chart(1, 0, 1),   Chart(2, 2, 3),  Chart(4, 4, 5),
                                  Chart(8, 6, 7),   Chart(16, 8, 9), Chart(32, 10, 11),
                                  Chart(64, 12, 13) ];
const KEY_VALUE_MAP : [[(u8,u8);14];4] = [[(b'`', b'~'),
                                           (b'1', b'!'),
                                           (b'2', b'@'),
                                           (b'3', b'#'),
                                           (b'4', b'$'),
                                           (b'5', b'%'),
                                           (b'6', b'^'),
                                           (b'7', b'&'),
                                           (b'8', b'*'),
                                           (b'9', b'('),
                                           (b'0', b')'),
                                           (b'-', b'_'),
                                           (b'=', b'+'),
                                           (KEY_BACKSPACE, KEY_BACKSPACE)],
                                           [(KEY_TAB, KEY_TAB),
                                           (b'q', b'Q'),
                                           (b'w', b'W'),
                                           (b'e', b'E'),
                                           (b'r', b'R'),
                                           (b't', b'T'),
                                           (b'y', b'Y'),
                                           (b'u', b'U'),
                                           (b'i', b'I'),
                                           (b'o', b'O'),
                                           (b'p', b'P'),
                                           (b'[', b'{'),
                                           (b']', b'}'),
                                           (b'\\',b'|')],
                                          [(KEY_FN, KEY_FN),
                                           (KEY_LEFT_SHIFT, KEY_LEFT_SHIFT),
                                           (b'a', b'A'),
                                           (b's', b'S'),
                                           (b'd', b'D'),
                                           (b'f', b'F'),
                                           (b'g', b'G'),
                                           (b'h', b'H'),
                                           (b'j', b'J'),
                                           (b'k', b'K'),
                                           (b'l', b'L'),
                                           (b';', b':'),
                                           (b'\'', b'\"'),
                                           (KEY_ENTER, KEY_ENTER)],
                                          [(KEY_LEFT_CTRL, KEY_LEFT_CTRL),
                                           (KEY_OPT, KEY_OPT),
                                           (KEY_LEFT_ALT, KEY_LEFT_ALT),
                                           (b'z', b'Z'),
                                           (b'x', b'X'),
                                           (b'c', b'C'),
                                           (b'v', b'V'),
                                           (b'b', b'B'),
                                           (b'n', b'N'),
                                           (b'm', b'M'),
                                           (b',', b'<'),
                                           (b'.', b'>'),
                                           (b'/', b'?'),
                                           (b' ', b' ')]];

pub struct KeysState {
    pub tab : bool,
    pub fn_mod : bool,
    pub shift : bool, 
    pub ctrl  : bool, 
    pub opt   : bool, 
    pub alt   : bool, 
    pub del   : bool, 
    pub enter : bool, 
    pub space : bool, 
    pub modifiers : u8,
    pub word : ConstGenericRingBuffer<u8,8>,
    pub hid_keys : ConstGenericRingBuffer<u8,8>,
    pub modifier_keys : ConstGenericRingBuffer<u8,8>,
}                     
impl KeysState {
    pub fn new() -> KeysState {
        KeysState {
            tab : false, 
            fn_mod : false, 
            shift: false, 
            ctrl: false,
            opt: false,
            alt: false,
            del: false,
            enter: false,
            space : false,
            modifiers: 0, 
            word: ConstGenericRingBuffer::<_,8>::new(),
            hid_keys: ConstGenericRingBuffer::<_,8>::new(),
            modifier_keys: ConstGenericRingBuffer::<_,8>::new(),
        }
    }
}
pub struct Keyboard {
    input_pins : [AnyPin<Input<PullUp>>;7],
    output_pins : [AnyPin<Output<PushPull>>;3],
    pub keys_state : KeysState,
    key_list_buffer : ConstGenericRingBuffer<(u8,u8),16>,
    key_pos_print_keys : ConstGenericRingBuffer<(u8,u8),8>,
    key_pos_hid_keys : ConstGenericRingBuffer<(u8,u8),8>,
    key_pos_modifier_keys : ConstGenericRingBuffer<(u8,u8),8>,
    pub caps_locked : bool,
    last_key_size : u8,
}

impl Keyboard {
    pub fn new(input_pins : [AnyPin<Input<PullUp>>;7], output_pins : [AnyPin<Output<PushPull>>;3]) -> Keyboard {
        Keyboard {
            keys_state: KeysState::new(),
            input_pins, output_pins,
            key_list_buffer : ConstGenericRingBuffer::<_,16>::new(),
            key_pos_print_keys : ConstGenericRingBuffer::<_,8>::new(),
            key_pos_hid_keys : ConstGenericRingBuffer::<_,8>::new(),
            key_pos_modifier_keys : ConstGenericRingBuffer::<_,8>::new(),
            caps_locked : false,
            last_key_size : 0,
        }
    }
    fn set_output(&mut self, out : u8) {
        if out & 0b0001 != 0 { self.output_pins[0].set_high().unwrap() } else { self.output_pins[0].set_low().unwrap() }
        if out & 0b0010 != 0 { self.output_pins[1].set_high().unwrap() } else { self.output_pins[1].set_low().unwrap() }
        if out & 0b0100 != 0 { self.output_pins[2].set_high().unwrap() } else { self.output_pins[2].set_low().unwrap() }        
    }
    fn read_input(&self) -> u8 {
        let mut buffer : u8 = 0;
        for i in 0.. 7 {
            let pin_value = if self.input_pins[i].is_high().unwrap() { 0x00 } else { 0x01} << i;
            buffer    = buffer | pin_value;
        }
        buffer
    }
    pub fn begin(&mut self ) { self.set_output(0) }
    pub fn get_key(&self ,key_coor : (u8,u8)) -> u8 {
        if self.keys_state.ctrl || self.keys_state.shift || self.caps_locked {
            return KEY_VALUE_MAP[key_coor.1 as usize][key_coor.0 as usize].1;
        } else {
            return KEY_VALUE_MAP[key_coor.1 as usize][key_coor.0 as usize].0;
        }
    }
    pub fn update_key_list(&mut self) { 
        self.key_list_buffer.clear();
        //Point2D_t coor;
        //uint8_t input_value = 0;
    
        for i in 0..8 {
            self.set_output( i);
            let input_value = self.read_input();
            if input_value != 0 {
                for j in 0..7 {
                    if input_value & (0x01 << j) != 0 {
                        let x = if i > 3 { X_MAP_CHART[j].1 } else { X_MAP_CHART[j].2 };
                        let y = 3 - if i > 3 { i - 4} else { i };            
                        self.key_list_buffer.push((x,y));
                    }
                }
            }
        }
    }
    pub fn get_key_value(&self, key_coor : (u8,u8)) -> (u8,u8) {
        KEY_VALUE_MAP[key_coor.1 as usize][key_coor.0 as usize]
    }
    pub fn is_pressed(&self) -> u8 { self.key_list_buffer.len() as u8 }
    pub fn is_change(&mut self) -> bool { 
        if self.last_key_size != self.key_list_buffer.len() as u8 {
            self.last_key_size = self.key_list_buffer.len() as u8;
            true
        } else {
            false
        }
    }
    pub fn is_key_pressed(&self, c : u8) -> bool{
        if !self.key_list_buffer.is_empty() {
            for &i in &self.key_list_buffer {
                if self.get_key(i) == c {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn update_keys_state(&mut self) { 
        self.keys_state = KeysState::new();
        self.key_pos_print_keys.clear();
        self.key_pos_hid_keys.clear();
        self.key_pos_modifier_keys.clear();
        for &ii in &self.key_list_buffer {
            let val = self.get_key_value(ii);
            match val.0 {
                KEY_FN => { self.keys_state.fn_mod = true; continue },
                KEY_OPT => { self.keys_state.opt = true; continue },
                KEY_LEFT_CTRL => {
                    self.keys_state.ctrl = true; 
                    self.key_pos_modifier_keys.push(ii);
                    continue 
                },
                KEY_LEFT_SHIFT => {
                    self.keys_state.shift = true; 
                    self.key_pos_modifier_keys.push(ii);
                    continue 
                },
                KEY_LEFT_ALT => {
                    self.keys_state.alt = true; 
                    self.key_pos_modifier_keys.push(ii);
                    continue 
                },
                KEY_TAB => {
                    self.keys_state.tab = true; 
                    self.key_pos_hid_keys.push(ii); 
                    continue 
                },
                KEY_BACKSPACE => {
                    self.keys_state.del = true; 
                    self.key_pos_hid_keys.push(ii);
                    continue 
                },
                KEY_ENTER => {
                    self.keys_state.enter = true; 
                    self.key_pos_hid_keys.push(ii);
                    continue 
                },
                _ => {
                    if val.0 == b' ' {
                        self.keys_state.space = true
                    }
                    self.key_pos_hid_keys.push(ii);
                    self.key_pos_print_keys.push(ii); 
                }
            }
        }
        for &i in &self.key_pos_modifier_keys {
            let key = self.get_key_value(i);
            self.keys_state.modifier_keys.push(key.0);
            self.keys_state.modifiers |= 1 << (key.0 - 0x80);
        }
        for &i in &self.key_pos_hid_keys {
            let mut k : u8 = self.get_key_value(i).0;
            if !(k == KEY_TAB || k == KEY_BACKSPACE || k == KEY_ENTER) {
                k = KB_ASCIIMAP[k as usize];
            }
            self.keys_state.hid_keys.push(k);
        }
        for &i in &self.key_pos_print_keys {
            let val = self.get_key_value(i);
            if self.keys_state.ctrl || self.keys_state.shift || self.caps_locked {
                self.keys_state.word.push(val.1);
            } else {
                self.keys_state.word.push(val.0);
            }
        }
    }
}