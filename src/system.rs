
use crate::sound::Sound;
use crate::display::Display;
use crate::keyboard::Keyboard;
use hal::Rng;
use hal::gpio::{GpioPin, Output, Input, PushPull, PullUp};
use hal::peripherals::{Peripherals, TIMG0};
use hal::timer::TimerGroup;
use hal::{clock::ClockControl, prelude::*, Delay, gpio::GpioExt, gdma::Gdma };
const TICKS_PER_FRAME : u64 = 1200000;
pub struct Cardputer<'d> {
    pub sound : Sound<'d>,
    pub display : Display<'d, GpioPin<Output<PushPull>,33>,GpioPin<Output<PushPull>,34>,GpioPin<Output<PushPull>,38>>,
    pub keyboard: Keyboard,
    pub rng : Rng,
    pub delay: Delay,
    pub timer: TimerGroup<'d,TIMG0>,
    pub old_time: u64,
    button_pin: GpioPin<Input<PullUp>,0>,
    old_button: bool,
}
impl <'d> Cardputer<'d> {
    pub fn new() -> Cardputer<'d> {
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let rng = Rng::new(peripherals.RNG);
        let clocks = ClockControl::max(system.clock_control).freeze();
        let delay = Delay::new(&clocks);
        let gp = peripherals.GPIO.split();
        let mut timer = TimerGroup::new(peripherals.TIMG0,&clocks);
        timer.timer0.set_counter_active(true);
        let keyboard = Keyboard::new(
            [ gp.gpio13.into_pull_up_input().degrade(), 
              gp.gpio15.into_pull_up_input().degrade(), 
              gp.gpio3.into_pull_up_input().degrade(), 
              gp.gpio4.into_pull_up_input().degrade(),
              gp.gpio5.into_pull_up_input().degrade(),
              gp.gpio6.into_pull_up_input().degrade(),
              gp.gpio7.into_pull_up_input().degrade() ],
            [ gp.gpio8.into_push_pull_output().degrade(), 
              gp.gpio9.into_push_pull_output().degrade(), 
              gp.gpio11.into_push_pull_output().degrade()]);
        let dma = Gdma::new(peripherals.DMA);
        let display = Display::new(peripherals.SPI2,dma.channel0,
            gp.gpio33.into_push_pull_output(),
            gp.gpio34.into_push_pull_output(),
            gp.gpio38.into_push_pull_output(),
            gp.gpio35.into_push_pull_output(),
            gp.gpio36.into_push_pull_output(),
            gp.gpio37.into_push_pull_output(),
            &clocks);
        let sound = Sound::new(peripherals.I2S0,dma.channel1,gp.gpio41,gp.gpio43,gp.gpio42,&clocks);
        Cardputer {
            keyboard, rng,
            button_pin: gp.gpio0.into_pull_up_input(),old_button: false,
            display, sound, delay, timer, old_time: 0
        }
    }

    pub fn button_pressed(&mut self) -> bool {
        let b = self.button_pin.is_low().unwrap();
        let b2 = b != self.old_button;
        self.old_button = b;
        return b && b2;
    }

    pub fn tick<T>(&mut self, synth: &mut T) -> bool where T : Iterator<Item =i16> {        
        let t = self.timer.timer0.now();
        let mut ret = false;
        if t > TICKS_PER_FRAME {
            self.display.flip();
            self.keyboard.update_key_list();
            self.keyboard.update_keys_state();
            self.timer.timer0.reset_counter();
            ret = true
        }
        self.sound.push_from_softsynth(synth);
        ret
    }
}
