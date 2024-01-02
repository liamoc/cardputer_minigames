use byteorder::{WriteBytesExt, LE};
use softsynth::{mix, Adsr, Oscillator, Sound, MAX_VOL, RATE};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut oscillator = Adsr::new(Oscillator::default(), 10, 300, MAX_VOL / 3 * 2, 10);
    oscillator.set_vol(MAX_VOL / 4);
    let theme = oscillator.into_player(&softsynth::songs::FRERE_JACQUES);
    let one_bar = theme.len() / 4;
    let len = theme.len() * 2 + 3 * one_bar;

    let theme1 = theme.clone().chain(theme.clone());
    let theme2 = (0..one_bar)
        .map(|_| 0)
        .chain(theme.clone())
        .chain(theme.clone());
    let theme3 = (0..one_bar * 2)
        .map(|_| 0)
        .chain(theme.clone())
        .chain(theme.clone());
    let theme4 = (0..one_bar * 3)
        .map(|_| 0)
        .chain(theme.clone())
        .chain(theme.clone());

    let v = mix(mix(theme1, theme2), mix(theme3, theme4));

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    stdout.write_all(b"RIFF")?;
    stdout.write_u32::<LE>(16 + 8 + 8 + 4 + len as u32 * 2)?;
    stdout.write_all(b"WAVE")?;

    stdout.write_all(b"fmt ")?;
    stdout.write_u32::<LE>(16)?;
    stdout.write_u16::<LE>(1)?; // PCM
    stdout.write_u16::<LE>(1)?; // mono
    stdout.write_u32::<LE>(RATE)?; // freq
    stdout.write_u32::<LE>(RATE * 1 * 16 / 8)?; // bytes/s
    stdout.write_u16::<LE>(1 * 16 / 8)?; // bytes/block
    stdout.write_u16::<LE>(16)?; // bits/sample

    stdout.write_all(b"data")?;
    stdout.write_u32::<LE>(len as u32 * 2)?;
    for s in v {
        stdout.write_i16::<LE>(s)?;
    }

    Ok(())
}
