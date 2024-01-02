#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use softsynth::{Adsr, Oscillator, Sound, MAX_VOL, RATE};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{gpio, pwm, stm32};

struct PwmSoundCard<Pwm1, Pwm2> {
    pwm1: Pwm1,
    pwm2: Pwm2,
}
impl<Pwm1, Pwm2> PwmSoundCard<Pwm1, Pwm2>
where
    Pwm1: embedded_hal::PwmPin,
    Pwm1::Duty: From<u16> + Into<u32>,
    Pwm2: embedded_hal::PwmPin,
    Pwm2::Duty: From<u16> + Into<u32>,
{
    fn set(&mut self, val: i16) {
        if val > 0 {
            let max = self.pwm1.get_max_duty().into();
            let duty = val as u32 * max / core::i16::MAX as u32;
            self.pwm1.set_duty(Pwm1::Duty::from(duty as u16));
            self.pwm2.set_duty(Pwm2::Duty::from(0));
        } else {
            let max = self.pwm2.get_max_duty().into();
            let duty = (-val) as u32 * max / core::i16::MAX as u32;
            self.pwm1.set_duty(Pwm1::Duty::from(0));
            self.pwm2.set_duty(Pwm2::Duty::from(duty as u16));
        }
    }
}

type SoundCard = PwmSoundCard<pwm::Pwm<stm32::TIM2, pwm::C1>, pwm::Pwm<stm32::TIM2, pwm::C2>>;

struct Context {
    sound_card: SoundCard,
    oscillator: Adsr<Oscillator>,
    button0: gpio::gpiob::PB12<gpio::Input<gpio::PullUp>>,
    button1: gpio::gpiob::PB13<gpio::Input<gpio::PullUp>>,
    button2: gpio::gpiob::PB14<gpio::Input<gpio::PullUp>>,
    button3: gpio::gpiob::PB15<gpio::Input<gpio::PullUp>>,
    button4: gpio::gpioa::PA8<gpio::Input<gpio::PullUp>>,
    button5: gpio::gpioa::PA9<gpio::Input<gpio::PullUp>>,
}
static mut CONTEXT: Option<Context> = None;

#[entry]
fn main() -> ! {
    let device = stm32::Peripherals::take().unwrap();
    let mut core = cortex_m::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let mut afio = device.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = device.GPIOA.split(&mut rcc.apb2);
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .freeze(&mut flash.acr);
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let c4 = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = device.TIM2.pwm(
        (c1, c2, c3, c4),
        &mut afio.mapr,
        (72_000_000 / 256).hz(),
        clocks,
        &mut rcc.apb1,
    );

    pwm.0.enable();
    pwm.1.enable();
    let sound_card = PwmSoundCard {
        pwm1: pwm.0,
        pwm2: pwm.1,
    };

    let mut oscillator = Adsr::new(Oscillator::default(), 100, 1000, MAX_VOL / 3 * 2, 2000);
    oscillator.set_freq(440);

    let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
    let button0 = gpiob.pb12.into_pull_up_input(&mut gpiob.crh);
    let button1 = gpiob.pb13.into_pull_up_input(&mut gpiob.crh);
    let button2 = gpiob.pb14.into_pull_up_input(&mut gpiob.crh);
    let button3 = gpiob.pb15.into_pull_up_input(&mut gpiob.crh);
    let button4 = gpioa.pa8.into_pull_up_input(&mut gpioa.crh);
    let button5 = gpioa.pa9.into_pull_up_input(&mut gpioa.crh);

    let context = Context {
        sound_card,
        oscillator,
        button0,
        button1,
        button2,
        button3,
        button4,
        button5,
    };
    unsafe {
        CONTEXT = Some(context);
    }

    core.SYST.set_clock_source(SystClkSource::Core);
    core.SYST.set_reload(72_000_000 / RATE);
    core.SYST.enable_interrupt();
    core.SYST.enable_counter();

    loop {
        cortex_m::asm::wfi();
    }
}

struct Unbouncer {
    cur: u16,
    new: u16,
    nb_changes: u32,
}
impl Unbouncer {
    fn is_change(&mut self, new: u16) -> bool {
        if new == self.cur {
            self.nb_changes = 0;
            false
        } else if new != self.new {
            self.new = new;
            self.nb_changes = 0;
            false
        } else if self.nb_changes > RATE * 50 / 1000 {
            self.cur = new;
            self.nb_changes = 0;
            true
        } else {
            self.nb_changes += 1;
            false
        }
    }
}

#[exception]
fn SysTick() {
    static mut UNBOUNCER: Unbouncer = Unbouncer {
        cur: 0,
        new: 0,
        nb_changes: 0,
    };
    let context = unsafe { CONTEXT.as_mut().unwrap() };

    context.sound_card.set(context.oscillator.get());
    let base = 262;
    let overtone = context.button0.is_low() as u32
        + context.button1.is_low() as u32 * 2
        + context.button2.is_low() as u32 * 4;
    let freq = overtone * base;
    let nb_half_pitch = context.button3.is_low() as u32 * 2
        + context.button4.is_low() as u32
        + context.button5.is_low() as u32 * 3;
    let freq = match nb_half_pitch {
        0 => freq,
        1 => freq * 9439 / 10000,
        2 => freq * 8909 / 10000,
        3 => freq * 8409 / 10000,
        4 => freq * 7937 / 10000,
        5 => freq * 7492 / 10000,
        6 => freq * 7071 / 10000,
        _ => unreachable!(),
    } as u16;

    if UNBOUNCER.is_change(freq) {
        if freq == 0 {
            context.oscillator.stop();
        } else {
            context.oscillator.set_freq(freq);
        }
    }
    context.oscillator.advance();
}
