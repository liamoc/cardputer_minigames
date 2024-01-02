#![no_std]
#![no_main]
mod keyboard;
mod display;
mod tiles;
mod system;
mod sound;

mod archaeologist;
mod daleks;
use esp_backtrace as _;
use esp_println::println;
use hal::entry;
use system::Cardputer;

#[entry]
fn main() -> ! {
    let mut sys = Cardputer::new();
    println!("Hello world!");
    
    loop {
        archaeologist::minesweeper(&mut sys);
     daleks::robots(&mut sys);
        println!("BUTTON");
    }
    
}
