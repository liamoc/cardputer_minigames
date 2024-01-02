use crate::{compute_ratio, MAX_VOL, RATE};

#[derive(Clone)]
pub struct Oscillator {
    pub sample: &'static [i16; 256],
    vol: i16,
    freq: u16,
    step: u8,
    modulo: u32,
    cur_idx: u8,
    cur_mod: u32,
}
impl crate::Sound for Oscillator {
    fn vol(&self) -> i16 {
        self.vol
    }
    fn set_freq(&mut self, freq: u16) {
        self.freq = freq;
        self.step = (256 * freq as u32 / RATE) as u8;
        self.modulo = 256 * freq as u32 % RATE;
    }
    fn set_vol(&mut self, vol: i16) {
        self.vol = vol;
    }
    fn get(&self) -> i16 {
        let before = self.sample[self.cur_idx as usize];
        let after = self.sample[self.cur_idx.wrapping_add(1) as usize];
        let res = compute_ratio(before, after, self.cur_mod, RATE);
        (res as i32 * self.vol as i32 / MAX_VOL as i32) as i16
    }
    fn advance(&mut self) {
        let modulo = self.cur_mod + self.modulo;
        self.cur_idx = (self.cur_idx as u32 + self.step as u32 + modulo / RATE) as u8;
        self.cur_mod = modulo % RATE;
    }
    fn stop(&mut self) {
        self.step = 0;
        self.modulo = 0;
        self.cur_idx = 0;
        self.cur_mod = 0;
    }
}
impl Oscillator {
    pub fn freq(&self) -> u16 {
        self.freq
    }
    
    pub fn noise() -> Self {
        Self {
            sample: &NOISE,
            freq: 0,
            vol: MAX_VOL,
            step: 0,
            modulo: 0,
            cur_idx:0,
            cur_mod: 0
        }
    }
}
impl Default for Oscillator {
    fn default() -> Self {
        Self {
            sample: &SIN,
            freq: 0,
            vol: MAX_VOL,
            step: 0,
            modulo: 0,
            cur_idx: 0,
            cur_mod: 0,
        }
    }
}

pub static SIN: [i16; 256] = [
    0, 804, 1607, 2410, 3211, 4011, 4807, 5601, 6392, 7179, 7961, 8739, 9511, 10278, 11038, 11792,
    12539, 13278, 14009, 14732, 15446, 16150, 16845, 17530, 18204, 18867, 19519, 20159, 20787,
    21402, 22004, 22594, 23169, 23731, 24278, 24811, 25329, 25831, 26318, 26789, 27244, 27683,
    28105, 28510, 28897, 29268, 29621, 29955, 30272, 30571, 30851, 31113, 31356, 31580, 31785,
    31970, 32137, 32284, 32412, 32520, 32609, 32678, 32727, 32757, 32767, 32757, 32727, 32678,
    32609, 32520, 32412, 32284, 32137, 31970, 31785, 31580, 31356, 31113, 30851, 30571, 30272,
    29955, 29621, 29268, 28897, 28510, 28105, 27683, 27244, 26789, 26318, 25831, 25329, 24811,
    24278, 23731, 23169, 22594, 22004, 21402, 20787, 20159, 19519, 18867, 18204, 17530, 16845,
    16150, 15446, 14732, 14009, 13278, 12539, 11792, 11038, 10278, 9511, 8739, 7961, 7179, 6392,
    5601, 4807, 4011, 3211, 2410, 1607, 804, 0, -804, -1607, -2410, -3211, -4011, -4807, -5601,
    -6392, -7179, -7961, -8739, -9511, -10278, -11038, -11792, -12539, -13278, -14009, -14732,
    -15446, -16150, -16845, -17530, -18204, -18867, -19519, -20159, -20787, -21402, -22004, -22594,
    -23169, -23731, -24278, -24811, -25329, -25831, -26318, -26789, -27244, -27683, -28105, -28510,
    -28897, -29268, -29621, -29955, -30272, -30571, -30851, -31113, -31356, -31580, -31785, -31970,
    -32137, -32284, -32412, -32520, -32609, -32678, -32727, -32757, -32767, -32757, -32727, -32678,
    -32609, -32520, -32412, -32284, -32137, -31970, -31785, -31580, -31356, -31113, -30851, -30571,
    -30272, -29955, -29621, -29268, -28897, -28510, -28105, -27683, -27244, -26789, -26318, -25831,
    -25329, -24811, -24278, -23731, -23169, -22594, -22004, -21402, -20787, -20159, -19519, -18867,
    -18204, -17530, -16845, -16150, -15446, -14732, -14009, -13278, -12539, -11792, -11038, -10278,
    -9511, -8739, -7961, -7179, -6392, -5601, -4807, -4011, -3211, -2410, -1607, -804,
];
pub static NOISE: [i16;256] = [16976, -17735, 27395, -25149, 589, 3715, 26716, 28236, 29205, -23369, -4594, -11937, 32396, -27827, -12663, 29591, 19958, 7218, 10365, 25865, -27243, 24311, -27550, 20205, 31349, 4028, -17996, -8448, -19211, -19965, 30262, 27220, -17410, -11664, 25350, 11280, 13372, 7813, 32688, -9931, -21165, -9972, 2661, 12112, 13941, -17964, 10150, -15442, 5956, -17403, 18637, -30092, -20612, -8234, -18789, 10415, -28741, 4537, 4751, 7744, -16321, 17872, 28241, 1766, -11715, -17161, 22233, -18197, 25409, -12395, -32434, -2999, -25025, 5227, 31998, -23578, -10917, 26685, 32668, -10194, -27168, -32026, 27307, -21595, 11909, 5121, 7307, -22395, -1746, 18244, -13501, 20697, -17088, -22027, -15524, -25314, -8836, -15657, 24857, 23900, 7041, 20677, 16131, -29661, -18257, 13930, 20098, 17149, -23038, -15150, -4672, 27422, -16016, -27931, 31828, -30259, 32582, -6779, -21751, -2946, -4066, 29231, 7653, 11081, -30208, 179, 27469, -3714, -5003, -5755, 25069, 9514, 30501, -16457, 9181, 23699, 12994, 5601, -20232, -22589, 26728, -16127, -32618, -26269, 19686, -19764, -28486, 30621, 2623, -27821, 12269, -15049, 16682, 29366, 21419, -18376, 14087, 17527, 23296, 30749, -21254, -467, -14307, -16376, 24411, 28292, -32032, 780, -6398, -7308, 2220, -14778, 14818, 24888, 5385, 5524, -4349, 30893, 7230, -2651, 4364, -18436, -7824, -992, 24147, 17185, 5665, 28621, -18259, -12727, 6876, -26082, -30879, -14576, 4006, -18626, 11374, 16134, 8389, 11529, -13123, 27171, 10254, -360, 21966, -3019, 24028, 7190, 25394, -16273, -29978, 15087, 15, 15715, -7622, -31061, -25988, -526, -14704, -7066, 20954, -28109, 31255, 1846, 8566, 17469, -17831, 11317, -29338, -14253, 7825, -30452, 28801, 23830, -6351, 15960, 31126, 18109, -31462, 4588, 11637, 14275, -18189, -21958, -8853, 13274, 8555, -32665, -11466, 4604, -4162, -1852, -7985, -23289, -4608, -4389];