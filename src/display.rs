
use hal::clock::Clocks;
use hal::gpio::OutputPin;
use esp_println::println;
use hal::peripheral::Peripheral;
use mipidsi::{Builder, Orientation,models::ST7789};
use spi_dma_displayinterface::spi_dma_displayinterface::new_no_cs;
use hal::spi::{master::*, master::prelude::*,SpiMode,FullDuplexMode};
use hal::dma::DmaPriority;
use hal::gdma::{Channel0, ChannelCreator0};
use hal::prelude::*;
use hal::Delay;
use hal::peripherals::SPI2;
use embedded_graphics::{draw_target::DrawTarget,pixelcolor::Rgb565, prelude::{RgbColor, Point},primitives::Rectangle};
use embedded_graphics_framebuf::FrameBuf;
use spi_dma_displayinterface::spi_dma_displayinterface::SPIInterface;
use hal::gpio::{Output,PushPull,GpioPin};

static mut DATA : [Rgb565;135*240] = [Rgb565::BLACK; 135 * 240];
static mut DESCRIPTORS : [u32;64*3] = [0u32; 64 * 3];
static mut RX_DESCRIPTORS : [u32;64*3] = [0u32; 64 * 3];

pub struct Display<'d,RST,DC,BL> 
where 
    DC: OutputPin + _embedded_hal_digital_v2_OutputPin,
    RST: OutputPin + _embedded_hal_digital_v2_OutputPin,
    BL: OutputPin + _embedded_hal_digital_v2_OutputPin,
{    
    pub display : mipidsi::Display<SPIInterface<'d, DC, GpioPin<Output<PushPull>, 0>, SPI2, Channel0, FullDuplexMode>, ST7789, RST>,
    pub fb :  FrameBuf<Rgb565, &'static mut [Rgb565; 240*135]>,    
    backlight :BL,
}

impl <'d,RST,DC,BL>  Display<'d,RST,DC,BL> 
where  
    DC: OutputPin + _embedded_hal_digital_v2_OutputPin,
    RST: OutputPin + _embedded_hal_digital_v2_OutputPin,
    BL: OutputPin + _embedded_hal_digital_v2_OutputPin,
{
    pub fn flip(&mut self) {
        let area = Rectangle::new(Point::new(0,0),self.fb.size());
        unsafe { self.display.fill_contiguous(&area,DATA).unwrap(); }
    }
    pub fn backlight_on(&mut self) {
        unsafe { self.backlight.set_high().unwrap_unchecked() };
    }
    pub fn backlight_off(&mut self) {
        unsafe { self.backlight.set_low().unwrap_unchecked() };
    }
    pub fn new<MOSI : Peripheral + 'd,SCK : Peripheral + 'd,CS : Peripheral + 'd>(spi : SPI2, dma : ChannelCreator0, rst: RST, rs : DC, bl : BL, mosi : MOSI, sck : SCK, cs : CS, clocks : &Clocks) -> Self  where 
        <MOSI as Peripheral>::P: OutputPin,
        <SCK as Peripheral>::P: OutputPin,
        <CS as Peripheral>::P: OutputPin, 
    {
        //MAGIC PRINT STATEMENT IS NEEDED TO STOP THE BUFFERS BEING PLACED IN FLASH
        unsafe { println!(" {:p} {:p} {:p}", &DATA, &DESCRIPTORS, &RX_DESCRIPTORS);}
        let mut delay = Delay::new(&clocks);
        let sp = unsafe { Spi::new(spi, 65000u32.kHz(), SpiMode::Mode0, &clocks)
            .with_mosi(mosi)
            .with_sck(sck)
            .with_cs(cs)
            .with_dma(dma.configure(
                false,
                &mut DESCRIPTORS,
                &mut RX_DESCRIPTORS,
                DmaPriority::Priority1,
            )) 
        };
        let spi = new_no_cs(135*240*2,sp,rs);
        let display = unsafe { 
            Builder::st7789_pico1(spi)
                .with_orientation(Orientation::Landscape(false))
                .init(&mut delay, Some(rst))
                .unwrap_unchecked() 
        };
        let fb = unsafe {
            FrameBuf::new(&mut DATA, 240, 135)
        };
        Display {
            display, backlight: bl, fb
        }
    }
}
