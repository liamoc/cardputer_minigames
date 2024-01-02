use crate::{compute_ratio, Sound, MAX_VOL, RATE};

#[derive(Debug, Clone)]
pub struct Adsr<S> {
    sound: S,
    attack_ms: u32,
    decay_ms: u32,
    sustain: i16,
    release_ms: u32,
    vol: i16,
    sustain_vol: i16,
    state: AdsrState,
}
#[derive(Copy, Clone, Debug)]
enum AdsrState {
    Stop,
    Attack { from_vol: i16, ticks: u32 },
    Decay(u32),
    Sustain,
    Release { from_vol: i16, ticks: u32 },
}

fn as_ticks(ms: u32) -> u32 {
    ms * RATE / 1000
}
impl<S: Sound> Adsr<S> {
    pub fn new(mut sound: S, attack_ms: u32, decay_ms: u32, sustain: i16, release_ms: u32) -> Self {
        sound.set_vol(0);
        sound.stop();
        let mut res = Self {
            sound,
            attack_ms,
            decay_ms,
            sustain,
            release_ms,
            vol: MAX_VOL,
            sustain_vol: 0,
            state: AdsrState::Stop,
        };
        res.set_vol(MAX_VOL);
        res
    }
}

impl<S: Sound> Sound for Adsr<S> {
    fn vol(&self) -> i16 {
        self.vol
    }
    fn set_freq(&mut self, freq: u16) {
        self.sound.set_freq(freq);
        self.state = AdsrState::Attack {
            from_vol: self.sound.vol(),
            ticks: as_ticks(self.attack_ms),
        };
    }
    fn get(&self) -> i16 {
        self.sound.get()
    }
    fn advance(&mut self) {
        use AdsrState::*;
        match self.state {
            Stop => {}
            Attack { from_vol, ticks } => {
                let vol = compute_ratio(self.vol, from_vol, ticks, as_ticks(self.attack_ms));
                self.sound.set_vol(vol);
                self.state = match ticks {
                    0 => Decay(as_ticks(self.decay_ms)),
                    ticks => Attack {
                        from_vol,
                        ticks: ticks - 1,
                    },
                }
            }
            Decay(ticks) => {
                let vol = compute_ratio(self.sustain_vol, self.vol, ticks, as_ticks(self.decay_ms));
                self.sound.set_vol(vol);
                self.state = match ticks {
                    0 => Sustain,
                    ticks => Decay(ticks - 1),
                }
            }
            Sustain => {}
            Release { from_vol, ticks } => {
                let vol = compute_ratio(0, from_vol, ticks, as_ticks(self.release_ms));
                self.sound.set_vol(vol);
                self.state = match ticks {
                    0 => {
                        self.sound.stop();
                        Stop
                    }
                    ticks => Release {
                        from_vol,
                        ticks: ticks - 1,
                    },
                }
            }
        }
        self.sound.advance();
    }
    fn stop(&mut self) {
        self.state = AdsrState::Release {
            from_vol: self.sound.vol(),
            ticks: as_ticks(self.release_ms),
        }
    }
    fn set_vol(&mut self, vol: i16) {
        self.vol = vol;
        self.sustain_vol = (self.vol as i32 * self.sustain as i32 / MAX_VOL as i32) as i16
    }
}
