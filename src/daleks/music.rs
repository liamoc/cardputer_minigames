
use softsynth::{*, songs::Score, pitch::*};

pub fn bass_osc() -> Adsr<Oscillator> {
    let mut osc = Adsr::new(Oscillator::default(), 10, 300, MAX_VOL / 4 * 2, 10);
    osc.set_vol(MAX_VOL - (MAX_VOL/24));
    osc
}
pub fn treble_osc() -> Adsr<Oscillator> {
    let mut osc = Adsr::new(Oscillator::default(), 300, 400, MAX_VOL / 2 * 2, 100);
    osc.set_vol(MAX_VOL/24);
    osc
}
pub const WON_SFX : Score = Score {
    tempo: 120/2,
    notes: &[(G4,1,16,95), (B4,1,16,95),(D5,1,16,95)],
};
pub const DED_SFX : Score = Score {
    tempo: 120/2,
    notes: &[(A0,1,4,95)],
};
pub const EXP_SFX : Score = Score {
    tempo: 120/2,
    notes: &[(A0,1,16,95)],
};
pub const DUD_SFX : Score = Score {
    tempo: 120/2,
    notes: &[(A3,1,16,95)],
};
pub const EMP_SFX : Score = Score {
    tempo: 120/2,
    notes: &[(A3,1,16,95), (AS3,1,16,95),(B3,1,16,95),(C4,1,16,95),(CS4,1,16,95)],
};
pub const TEL_SFX : Score = Score {
    tempo: 120/2,
    notes: &[(A4,1,16,95), (AS4,1,16,95),(B4,1,16,95),(C5,1,16,95),(CS5,1,16,95)],
};

pub const BASS_END : Score = Score {
    tempo: 120/4,
    notes: &[
    (E3,1,4,95),
    (E3,1,8,95),
    (E3,3,8,60),
    ]
};
pub const BASS_1 : Score = Score {
    tempo: 120/4,
    notes: &[
    (E3,1,4,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (E3,1,4,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (E3,1,4,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (G3,1,4,95),
    (G3,1,8,95),
    (G3,3,8,60),        
    ]
};
pub const BASS_2 : Score = Score {
    tempo: 120/4,
    notes: &[
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (G3,1,4,95),
    (G3,1,4,95),
    (FS3,1,8,95),
    (D3,1,8,95),
    ]
};
pub const BASS_3 : Score = Score {
    tempo: 120/4,
    notes: &[
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    (B2,1,4,95),
    (B2,1,4,95),
    (C3,1,8,95),
    (D3,1,8,95),
    ]
};
pub const BASS_4 : Score = Score {
    tempo: 120/4,
    notes: &[
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (G3,1,8,95),
    (G3,1,8,95),
    (G3,1,8,95),
    (G3,3,8,60),
    (B2,1,4,95),
    (B2,1,4,95),
    (C3,1,8,95),
    (D3,1,8,95),
    ]
};
pub const BASS_5 : Score = Score {
    tempo: 120/4,
    notes: &[
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,1,8,95),
    (E3,3,8,60),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    ]
};
pub const BASS_6H : Score = Score {
    tempo: 120/4,
    notes: &[
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    (D3,1,4,95),
    (D3,1,4,95),
    (A2,1,4,95),
    ]
};
pub const BASS_6 : Score = Score {
    tempo: 120/4,
    notes: &[
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    (D3,1,4,95),
    (D3,1,4,95),
    (A2,1,4,95),
    ]
};
pub const BASS_7 : Score = Score {
    tempo: 120/4,
    notes: &[
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,3,8,60),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,3,8,60),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,3,8,60),
    (D3,2,8,95),
    (D3,1,8,95),
    (D3,3,8,60),
    ]
};
pub const BASS_8 : Score = Score {
    tempo: 120/4,
    notes: &[
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,3,8,60),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,1,8,95),
    (B2,3,8,60),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,1,8,95),
    (G2,3,8,60),
    ]
};
pub const TREBLE_1 : Score = Score {
    tempo: 120/4,
    notes: &[
    (A0,9,4,0),
    (B4,3,8,95),
    (C6,3,8,95),
    ]
};
pub const TREBLE_2 : Score = Score {
    tempo: 120/4,
    notes: &[
    (B5,9,4,98),
    (D6,3,8,95),
    (A4,3,8,95),
    ]
};
pub const TREBLE_3 : Score = Score {
    tempo: 120/4,
    notes: &[
    (B4,9,4,98),
    (D6,3,8,0),
    (A4,3,8,0),
    ]
};
pub const TREBLE_4 : Score = Score {
    tempo: 120/4,
    notes: &[
    (B5,3,8,98),
    (G5,3,8,98),
    (E5,3,8,98),
    (B4,3,8,98),
    (D5,5,8,98),
    (C5,1,8,98),
    (B4,5,8,98),
    (C5,1,8,98),
    ]
};
pub const TREBLE_5 : Score = Score {
    tempo: 120/4,
    notes: &[
    (B4,9,4,98),
    (B4,3,4,0),
    ]
};
pub const TREBLE_6 : Score = Score {
    tempo: 120/4,
    notes: &[
    (A0,9,4,0),
    (B4,3,8,0),
    (C6,3,8,95),
    ]
};
pub const TREBLE_7 : Score = Score {
    tempo: 120/4,
    notes: &[
    (B5,9,4,95),
    (G5,3,8,95),
    (B5,3,8,95),
    ]
};
pub const TREBLE_8 : Score = Score {
    tempo: 120/4,
    notes: &[
    (A5,2,4,95),
    (G5,1,8,95),
    (FS5,1,8,95),
    (G5,15,8,95),
    (D6,3,8,95),
    ]
};
pub const TREBLE_9 : Score = Score {
    tempo: 120/4,
    notes: &[
    (E6,2,4,95),
    (D6,1,8,95),
    (C6,1,8,95),
    (D6,3,8,95),
    (G5,3,8,95),
    (E6,2,4,95),
    (D6,1,8,95),
    (C6,1,8,95),
    (D6,3,8,95),
    (B5,3,8,95),
    ]
};
pub const TREBLE_10 : Score = Score {
    tempo: 120/4,
    notes: &[
    (A5,5,4,95),
    (G5,1,8,95),
    (FS5,1,8,95),
    (G5,6,4,95),
    ]
};


pub const BASS_PATCHES : &'static[&'static Score]   = &[&BASS_1    ,&BASS_2    ,&BASS_2  , &BASS_3  , &BASS_3  ,&BASS_4   , &BASS_5  ,&BASS_6H   ,&BASS_6  , &BASS_7,&BASS_7,   &BASS_8,  &BASS_3,  &BASS_3,   &BASS_5];
pub const TREBLE_PATCHES : &'static[&'static Score] = &[&EMPTY_SONG,&EMPTY_SONG,&TREBLE_1, &TREBLE_2, &TREBLE_3, &TREBLE_4, &TREBLE_5,&EMPTY_SONG,&TREBLE_6,&TREBLE_7,&TREBLE_8,&TREBLE_9,&TREBLE_10,&TREBLE_4,&TREBLE_5];

pub const EMPTY_SONG : Score = Score { tempo: 120/8, notes: &[]};