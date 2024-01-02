# softsynth

A sound synthetizer in no_std rust

## Goal

The goal of this crate is to produce music in a program, and that this program can be run on a microcontroller without FPU in realtime.

The constraints are:
 - must work in `#[no_std]`
 - no `f32` or `f64`
 - no costy computations (`ln`, `sin` are out of scope)

It can be used for fixed score playing or realtime sound generation.

## Binaries

There is (at the time of writing) 2 binaries:
 - an example generating a wav file
 - `bluepill-player` that run on a microcontroller and react to buttons to play music
