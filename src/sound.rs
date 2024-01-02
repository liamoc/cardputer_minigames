use hal::gdma::ChannelCreator1;
use hal::peripheral::Peripheral;
use hal::peripherals::I2S0;
use hal::dma::DmaPriority;
use hal::gpio::OutputPin;
use hal::{i2s::{I2s,I2sWriteDmaTransfer, I2sWriteDma,Standard, DataFormat}, gdma::Channel1};
use hal::prelude::*;
use hal::clock::Clocks;


static mut SOUND_BUF : [u8;32768] = [0;32768];
static mut DESCRIPTORS : [u32;64*3] = [0u32; 64 * 3];
static mut RX_DESCRIPTORS : [u32;64*3] = [0u32; 64 * 3];
pub struct Sound<'d> {
  filler : [u8;32768],
  transfer : I2sWriteDmaTransfer<'d, I2S0, Channel1 , &'static mut [u8;32768]>
}

impl <'d> Sound<'d> {
    pub fn new<BCLK : Peripheral + 'd,WS : Peripheral + 'd,DOUT : Peripheral + 'd>(i2s : I2S0,dma : ChannelCreator1,bclk: BCLK,ws:WS,dout:DOUT, clocks:&Clocks) -> Sound<'d> 
    where
        <BCLK as Peripheral>::P: OutputPin,
        <WS as Peripheral>::P: OutputPin,
        <DOUT as Peripheral>::P: OutputPin,
    {
        let i2s = unsafe {
            I2s::new(i2s, Standard::Philips, DataFormat::Data16Channel16, 44100u32.Hz(),
                dma.configure(
                    false,
                    &mut DESCRIPTORS,
                    &mut RX_DESCRIPTORS, 
                    DmaPriority::Priority0
                ), &clocks)
        };
        let i2s_tx = i2s.i2s_tx
            .with_bclk(bclk)
            .with_ws(ws)
            .with_dout(dout)
            .build();
        Sound {
            filler: [0;32768],
            transfer : unsafe { i2s_tx.write_dma_circular(&mut SOUND_BUF).unwrap_unchecked()}
        }
    }

    pub fn push_from_softsynth<T : Iterator<Item = i16>>(&mut self, sy : &mut T)  {
        let avail = self.transfer.available();
        if avail > 0 {
            let avail = usize::min(32768, avail);
            let mut bidx = 0;
            while bidx < avail {
                if let Some(val) = sy.next() {
                    let s : [u8;2] = (val/2).to_le_bytes();
                    self.filler[bidx] = s[0];
                    self.filler[bidx+1] = s[1];
                    bidx += 2;
                } else { 
                    self.filler[bidx] = 0;
                    self.filler[bidx+1] = 0;
                    bidx += 2;
                }
            }
            self.transfer.push(&self.filler[0..avail]).unwrap();
        }
    }
}