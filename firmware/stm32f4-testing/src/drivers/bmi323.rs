
pub mod registers;

use rtt_target::rprintln;
use stm32f4xx_hal::{
    pac::SPI1,
    spi::{Spi,Instance},
    gpio::{Pin, Output}
};

pub struct Bmi323<SPI,const A: char, const N: u8>
where
    SPI: Instance
{
    spi: Spi<SPI>,
    cs: Pin<A,N,Output>
}

impl<SPI,const A:char,const N:u8> Bmi323<SPI,A,N>
where
    SPI: Instance
{

    pub fn new(spi: Spi<SPI>, cs: Pin<A,N,Output>) -> Self{
        Self{spi,cs}
    }

}
impl Bmi323<SPI1,'A',4> {

    pub fn who_am_i(&mut self){
        let tx = [0x00 | 0x80, 0x00u8];
        let mut rx = [0x00;4];

        self.cs.set_low();
        self.spi.transfer(&mut rx, &tx).unwrap();
        self.cs.set_high();

        rprintln!("{:?}",rx)
    }

    pub fn read_accel_config(&mut self){
        let tx = [0x20 | 0x80, 0x00u8];
        let mut rx = [0x00;10];

        self.cs.set_low();
        self.spi.transfer(&mut rx, &tx).unwrap();
        self.cs.set_high();
        rprintln!("{:?}", rx)

    }

}