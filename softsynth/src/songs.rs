use crate::pitch::*;
use crate::Action;

#[derive(Clone, Debug)]
pub struct Score {
    pub tempo: u8,
    pub notes: &'static [(u16, u8, u8, u8)],
}
impl Score {
    pub fn events(&self) -> Events {
        Events {
            whole_ms: 60 * 1000 / (self.tempo as u32),
            notes: self.notes.iter(),
            rest: None,
        }
    }
    pub fn ms_events(&self) -> MsEvents {
        self.events().ms_events()
    }
    pub fn ms_duration(&self) -> u32 {
        self.events().map(|e| e.ms_duration()).sum()
    }
}
#[derive(Clone, Debug)]
pub struct Events {
    whole_ms: u32,
    notes: ::core::slice::Iter<'static, (u16, u8, u8, u8)>,
    rest: Option<u32>,
}
impl Events {
    pub fn ms_events(self) -> MsEvents {
        MsEvents {
            events: self,
            wait_ms: 0,
        }
    }
}
impl ::core::iter::Iterator for Events {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        use cast::u32;
        match self.rest {
            Some(ms) => {
                self.rest = None;
                Some(Event::Rest { ms })
            }
            None => self.notes.next().map(|&(pitch, n, d, pct)| {
                let ms = self.whole_ms * u32(n) / u32(d);
                let note_ms = ms * u32(pct) / 100;
                let rest_ms = ms * (100 - u32(pct)) / 100;
                if rest_ms > 0 {
                    self.rest = Some(rest_ms);
                }
                Event::Note { pitch, ms: note_ms }
            }),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Note { pitch: u16, ms: u32 },
    Rest { ms: u32 },
}
impl Event {
    pub fn ms_duration(&self) -> u32 {
        match *self {
            Event::Note { ms, .. } => ms,
            Event::Rest { ms, .. } => ms,
        }
    }
    pub fn to_action(&self) -> crate::Action {
        match *self {
            Event::Note { pitch, .. } => Action::Start(pitch),
            Event::Rest { .. } => Action::Stop,
        }
    }
}
#[derive(Clone, Debug)]
pub struct MsEvents {
    events: Events,
    wait_ms: u32,
}
impl ::core::iter::Iterator for MsEvents {
    type Item = MsEvent;
    fn next(&mut self) -> Option<Self::Item> {
        if self.wait_ms > 0 {
            self.wait_ms -= 1;
            return Some(MsEvent::Wait);
        }
        self.events.next().map(|e| match e {
            Event::Note { pitch, ms } => {
                self.wait_ms = ms;
                MsEvent::BeginNote { pitch }
            }
            Event::Rest { ms } => {
                self.wait_ms = ms;
                MsEvent::EndNote
            }
        })
    }
}
pub enum MsEvent {
    BeginNote { pitch: u16 },
    EndNote,
    Wait,
}

pub static AU_FEU_LES_POMPIERS: Score = Score {
    tempo: 120 / 4,
    notes: &AU_FEU_LES_POMPIERS_NOTES,
};
static AU_FEU_LES_POMPIERS_NOTES: [(u16, u8, u8, u8); 48] = [
    (G5, 1, 4, 90),
    (G5, 1, 4, 90),
    (B5, 1, 8, 90),
    (G5, 1, 8, 90),
    (D5, 1, 4, 90),
    (D5, 1, 8, 90),
    (D5, 1, 16, 90),
    (D5, 1, 16, 90),
    (D5, 1, 8, 90),
    (D5, 1, 8, 90),
    (B5, 1, 4, 90),
    (G5, 1, 4, 90),
    (G5, 1, 4, 90),
    (G5, 1, 4, 90),
    (B5, 1, 8, 90),
    (G5, 1, 8, 90),
    (D5, 1, 4, 90),
    (D5, 1, 8, 90),
    (D5, 1, 16, 90),
    (D5, 1, 16, 90),
    (D5, 1, 8, 90),
    (D5, 1, 8, 90),
    (G5, 1, 2, 90),
    (G5, 1, 8, 90),
    (G5, 1, 8, 90),
    (G5, 1, 8, 90),
    (B5, 1, 8, 90),
    (D6, 1, 8, 90),
    (B5, 1, 8, 90),
    (G5, 1, 4, 90),
    (D5, 1, 8, 90),
    (D6, 1, 8, 90),
    (D5, 1, 8, 90),
    (D6, 1, 8, 90),
    (B5, 1, 4, 90),
    (G5, 1, 4, 90),
    (G5, 1, 8, 90),
    (G5, 1, 8, 90),
    (G5, 1, 8, 90),
    (B5, 1, 8, 90),
    (D6, 1, 8, 90),
    (B5, 1, 8, 90),
    (G5, 1, 4, 90),
    (D5, 1, 8, 90),
    (D6, 1, 8, 90),
    (D5, 1, 8, 90),
    (D6, 1, 8, 90),
    (G5, 1, 2, 90),
];

pub static BATEAU_SUR_LEAU: Score = Score {
    tempo: 80 / 4,
    notes: &BATEAU_SUR_LEAU_NOTES,
};
static BATEAU_SUR_LEAU_NOTES: [(u16, u8, u8, u8); 23] = [
    (E5, 1, 4, 95),
    (C5, 1, 4, 95),
    (E5, 1, 4, 95),
    (C5, 1, 4, 95),
    (D5, 1, 8, 95),
    (E5, 1, 8, 95),
    (F5, 1, 8, 95),
    (E5, 1, 8, 95),
    (D5, 1, 8, 95),
    (G5, 1, 8, 95),
    (E5, 1, 8, 95),
    (C5, 1, 8, 95),
    (E5, 1, 4, 95),
    (C5, 1, 4, 95),
    (E5, 1, 4, 95),
    (C5, 1, 4, 95),
    (D5, 1, 8, 95),
    (E5, 1, 8, 95),
    (F5, 1, 8, 95),
    (E5, 1, 8, 95),
    (D5, 1, 8, 95),
    (G5, 1, 8, 95),
    (C5, 1, 4, 95),
];

pub static FRERE_JACQUES: Score = Score {
    tempo: 140 / 4,
    notes: &FRERE_JACQUES_NOTES,
};
static FRERE_JACQUES_NOTES: [(u16, u8, u8, u8); 32] = [
    (C5, 1, 4, 90),
    (D5, 1, 4, 90),
    (E5, 1, 4, 90),
    (C5, 1, 4, 90),
    (C5, 1, 4, 90),
    (D5, 1, 4, 90),
    (E5, 1, 4, 90),
    (C5, 1, 4, 90),
    (E5, 1, 4, 90),
    (F5, 1, 4, 90),
    (G5, 1, 2, 90),
    (E5, 1, 4, 90),
    (F5, 1, 4, 90),
    (G5, 1, 2, 90),
    (G5, 3, 16, 90),
    (A5, 1, 16, 90),
    (G5, 1, 8, 90),
    (F5, 1, 8, 90),
    (E5, 1, 4, 90),
    (C5, 1, 4, 90),
    (G5, 3, 16, 90),
    (A5, 1, 16, 90),
    (G5, 1, 8, 90),
    (F5, 1, 8, 90),
    (E5, 1, 4, 90),
    (C5, 1, 4, 90),
    (C5, 1, 4, 90),
    (G4, 1, 4, 90),
    (C5, 1, 2, 90),
    (C5, 1, 4, 90),
    (G4, 1, 4, 90),
    (C5, 1, 2, 90),
];

pub static IL_ETAIT_UN_PETIT_NAVIRE: Score = Score {
    tempo: 100 / 4,
    notes: &IL_ETAIT_UN_PETIT_NAVIRE_NOTES,
};
static IL_ETAIT_UN_PETIT_NAVIRE_NOTES: [(u16, u8, u8, u8); 74] = [
    (B5, 1, 8, 90),
    (B5, 1, 8, 90),
    (B5, 1, 8, 90),
    (D5, 1, 4, 90),
    (B5, 1, 4, 90),
    (C6, 1, 8, 90),
    (B5, 1, 8, 90),
    (B5, 1, 4, 90),
    (A5, 1, 8, 90),
    (A5, 1, 8, 90),
    (A5, 1, 8, 90),
    (A5, 1, 8, 90),
    (D5, 1, 4, 90),
    (A5, 1, 4, 90),
    (B5, 1, 8, 90),
    (A5, 1, 8, 90),
    (A5, 1, 4, 90),
    (G5, 1, 8, 90),
    (B5, 1, 8, 90),
    (B5, 1, 8, 90),
    (B5, 1, 8, 90),
    (B5, 1, 4, 90),
    (B5, 1, 4, 90),
    (B5, 1, 8, 90),
    (D6, 1, 8, 90),
    (C6, 1, 8, 90),
    (B5, 1, 8, 90),
    (A5, 1, 8, 90),
    (A5, 1, 8, 90),
    (A5, 1, 8, 90),
    (A5, 1, 8, 90),
    (A5, 1, 4, 90),
    (A5, 1, 4, 90),
    (A5, 1, 8, 90),
    (C6, 1, 8, 90),
    (B5, 1, 8, 90),
    (A5, 1, 8, 90),
    (G5, 1, 8, 90),
    (D5, 1, 8, 90),
    (G5, 1, 8, 90),
    (B5, 1, 8, 90),
    (D6, 1, 2, 90),
    (B5, 1, 4, 90),
    (D6, 1, 4, 90),
    (B5, 1, 4, 90),
    (D6, 1, 4, 90),
    (C6, 3, 16, 90),
    (B5, 1, 16, 90),
    (A5, 1, 2, 90),
    (A5, 3, 16, 90),
    (B5, 1, 16, 90),
    (C6, 3, 16, 90),
    (D6, 1, 16, 90),
    (E6, 1, 4, 90),
    (D6, 1, 4, 90),
    (E6, 1, 4, 90),
    (D6, 1, 4, 90),
    (B5, 3, 4, 90),
    (B5, 1, 4, 90),
    (D6, 1, 4, 90),
    (B5, 1, 4, 90),
    (D6, 1, 4, 90),
    (C6, 3, 16, 90),
    (B5, 1, 16, 90),
    (A5, 1, 2, 90),
    (A5, 3, 16, 90),
    (B5, 1, 16, 90),
    (C6, 3, 16, 90),
    (D6, 1, 16, 90),
    (E6, 1, 4, 90),
    (D6, 1, 4, 90),
    (E6, 1, 4, 90),
    (D6, 1, 4, 90),
    (G5, 5, 8, 90),
];

pub static LAVENTURIER: Score = Score {
    tempo: 160 / 4,
    notes: &LAVENTURIER_NOTES,
};
static LAVENTURIER_NOTES: [(u16, u8, u8, u8); 17] = [
    (A4, 1, 4, 95),
    (D5, 1, 8, 95),
    (E5, 1, 8, 95),
    (G5, 1, 4, 95),
    (E5, 1, 4, 95),
    (D5, 1, 8, 95),
    (C5, 1, 4, 95),
    (A4, 5, 8, 95),
    (C5, 1, 4, 95),
    (D5, 1, 8, 95),
    (E5, 1, 8, 95),
    (G5, 1, 4, 95),
    (E5, 1, 4, 95),
    (D5, 1, 8, 95),
    (E5, 1, 8, 95),
    (D5, 1, 8, 95),
    (E5, 5, 8, 95),
];

pub static MARIO_THEME_INTRO: Score = Score {
    tempo: 185 / 4,
    notes: &MARIO_THEME_INTRO_NOTES,
};
static MARIO_THEME_INTRO_NOTES: [(u16, u8, u8, u8); 7] = [
    (E5, 1, 8, 50),
    (E5, 1, 4, 25),
    (E5, 1, 4, 25),
    (C5, 1, 8, 50),
    (E5, 1, 4, 25),
    (G5, 1, 2, 25),
    (G4, 1, 2, 25),
];

pub static SO_WHAT: Score = Score {
    tempo: 120 / 8 * 3,
    notes: &SO_WHAT_NOTES,
};
static SO_WHAT_NOTES: [(u16, u8, u8, u8); 42] = [
    (D3, 2, 8, 0),
    (D3, 1, 8, 80),
    (A3, 2, 8, 80),
    (B3, 1, 8, 80),
    (C4, 2, 8, 80),
    (D4, 1, 8, 80),
    (E4, 2, 8, 80),
    (C4, 1, 8, 30),
    (D4, 6, 8, 60),
    (E5, 5, 8, 100),
    (D5, 1, 8, 50),
    (D3, 2, 8, 0),
    (D3, 1, 8, 80),
    (A3, 2, 8, 80),
    (B3, 1, 8, 80),
    (C4, 2, 8, 80),
    (D4, 1, 8, 80),
    (E4, 2, 8, 80),
    (C4, 1, 8, 30),
    (D4, 2, 8, 80),
    (A3, 4, 8, 60),
    (E5, 5, 8, 100),
    (D5, 1, 8, 50),
    (D3, 2, 8, 0),
    (D3, 1, 8, 80),
    (A3, 2, 8, 80),
    (B3, 1, 8, 80),
    (C4, 2, 8, 80),
    (D4, 1, 8, 80),
    (E4, 2, 8, 80),
    (C4, 1, 8, 30),
    (D4, 6, 8, 60),
    (E5, 5, 8, 100),
    (D5, 1, 8, 50),
    (E4, 2, 8, 00),
    (E4, 4, 8, 80),
    (E4, 3, 8, 80),
    (E4, 3, 8, 80),
    (D4, 5, 8, 80),
    (A3, 1, 8, 100),
    (E5, 5, 8, 100),
    (D5, 1, 8, 50),
];

pub static THIRD_KIND: Score = Score {
    tempo: 120 / 4,
    notes: &THIRD_KIND_NOTES,
};
static THIRD_KIND_NOTES: [(u16, u8, u8, u8); 6] = [
    (BF5, 1, 4, 100),
    (C6, 1, 4, 100),
    (AF5, 1, 4, 100),
    (AF4, 1, 4, 100),
    (EF5, 1, 2, 100),
    (BF5, 1, 2, 0),
];
