pub mod commands;
pub mod setup;
pub mod registers;

use stm32f4xx_hal::gpio::{Pin, Output, PushPull};
use stm32f4xx_hal::pac::SPI2;
use stm32f4xx_hal::spi::{Spi};

pub struct Nrf24l01 {
    spi: Spi<SPI2>,
    ce: Pin<'A',10,Output<PushPull>>,
    csn: Pin<'A',11,Output<PushPull>>
}

impl Nrf24l01 {
    pub fn init(
        spi: Spi<SPI2>,
        ce: Pin<'A',10,Output<PushPull>>,
        csn: Pin<'A',11,Output<PushPull>>
    )-> Self{
        Self{
            spi,
            ce,
            csn
        }
    }
}

pub trait WriteRead{
    fn read_8bit(&mut self, addr: u8) -> u8;
    fn write_8bit(&mut self, addr: u8,data: u8);
    fn write_40bit(&mut self, addr: u8, data: [u8; 5]);
}

impl WriteRead for Nrf24l01 {
    fn read_8bit(&mut self, addr: u8) -> u8 {
        let mut rx = [0u8; 2];
        self.csn.set_low();
        self.spi.transfer(&mut rx,&[addr,0_u8]).unwrap();
        self.csn.set_high();
        rx[1]
    }

    fn write_8bit(&mut self, addr: u8, data: u8) {
        self.csn.set_low();
        self.spi.write(&[addr | (1<<5),data]).unwrap();
        self.csn.set_high();
    }
    ///[[msb, ... ,lsb]] format
    fn write_40bit(&mut self, addr: u8, data: [u8; 5]){
        self.csn.set_low();
        self.spi.write(&[addr | (1<<5),data[4],data[3],data[2],data[1],data[0]]).unwrap();
        self.csn.set_high();
    }
}

