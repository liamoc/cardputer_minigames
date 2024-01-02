#![no_std]

mod adsr;
mod oscillator;
pub mod pitch;
pub mod songs;

pub use adsr::Adsr;
pub use oscillator::Oscillator;

pub const RATE: u32 = 48000;
pub const MAX_VOL: i16 = core::i16::MAX;

pub enum Action {
    Vol(i16),
    Start(u16),
    Stop,
}

pub trait Sound {
    fn vol(&self) -> i16;
    fn get(&self) -> i16;
    fn advance(&mut self);
    fn step(&mut self) -> i16 {
        let res = self.get();
        self.advance();
        res
    }

    fn set_freq(&mut self, freq: u16);
    fn stop(&mut self);
    fn set_vol(&mut self, vol: i16);

    fn modify(&mut self, action: &Action) {
        match action {
            Action::Vol(vol) => self.set_vol(*vol),
            Action::Start(freq) => self.set_freq(*freq),
            Action::Stop => self.stop(),
        }
    }

    fn into_player(self, score: &songs::Score) -> Player<Self>
    where
        Self: Sized,
    {
        let mut events = score.events();
        Player {
            sound: self,
            t: score.ms_duration() * RATE / 1000,
            event: events.next(),
            events,
            next_ms: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player<S> {
    sound: S,
    event: Option<songs::Event>,
    events: songs::Events,
    t: u32,
    next_ms: u32,
}
impl<S: Sound> Player<S> {
    pub fn into_sound(self) -> S {
        self.sound
    }
}
impl<S: Sound> Iterator for Player<S> {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.t == 0 {
            return None;
        }
        if self.t % (RATE / 1000) == 0 {
            loop {
                match self.event {
                    None => break,
                    Some(_) if self.next_ms != 0 => break,
                    Some(e) => {
                        self.sound.modify(&e.to_action());
                        self.next_ms = e.ms_duration();
                    }
                }
                self.event = self.events.next();
            }
            self.next_ms -= 1;
        }
        self.t -= 1;
        Some(self.sound.step())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.t as usize, Some(self.t as usize))
    }
}
impl<S: Sound> core::iter::ExactSizeIterator for Player<S> {}

pub fn mix<I1, I2>(iter1: I1, iter2: I2) -> Mix<I1, I2> {
    Mix { iter1, iter2 }
}
#[derive(Clone)]
pub struct Mix<I1, I2> {
    iter1: I1,
    iter2: I2,
}
impl<I1, I2> ExactSizeIterator for Mix<I1,I2>
where 
    I1: ExactSizeIterator<Item = i16>,
    I2: ExactSizeIterator<Item = i16>,
{
    fn len(&self) -> usize {
        self.iter1.len().max(self.iter2.len())
    }

}

impl<I1, I2> Iterator for Mix<I1, I2>
where
    I1: Iterator<Item = i16>,
    I2: Iterator<Item = i16>,
{
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter1.next(), self.iter2.next()) {
            (None, None) => None,
            (Some(i), None) | (None, Some(i)) => Some(i),
            (Some(i1), Some(i2)) => Some(i1.saturating_add(i2)),
        }
    }
}

pub(crate) fn compute_ratio(from: i16, to: i16, num: u32, denom: u32) -> i16 {
    if denom == 0 {
        return from;
    }
    // Now, denom can't be 0

    // loose precision to not overflow
    let loose = 16u32.saturating_sub(denom.leading_zeros());
    let num = (num >> loose) as i32;
    let denom = (denom >> loose) as i32;

    ((to as i32 - from as i32) * num / denom + from as i32) as i16
}
