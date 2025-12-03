
pub mod constants;
use constants::Registers;
pub mod config;

use rtt_target::rprintln;
use stm32f4xx_hal::{
    pac::SPI1,
    spi::{Spi,Instance},
    gpio::{Pin, Output}
};
use crate::helper::U8ArrayWrapper;

pub struct Bmi323<SPI,const A: char, const N: u8>
where
    SPI: Instance
{
    spi: Spi<SPI>,
    cs: Pin<A,N,Output>,
}

impl<SPI,const A:char,const N:u8> Bmi323<SPI,A,N>
where
    SPI: Instance
{
    pub fn new(spi: Spi<SPI>, cs: Pin<A,N,Output>) -> Self{
        Self{ spi,cs }
    }
}


impl Bmi323<SPI1,'A',4> {

    pub fn who_am_i(&mut self){
        let buf = self._read_16bit_register(Registers::CHIP_ID);
        if buf[1] == 0x43{
            rprintln!("BMI323 detected");
        }else {
            rprintln!("BMI323 not detected"); }
    }

    pub fn read_gyr(&mut self){
        let gyr_x = U8ArrayWrapper{inner: self._read_16bit_register(Registers::GYR_DATA_X)};
        let gyr_y = U8ArrayWrapper{inner: self._read_16bit_register(Registers::GYR_DATA_X+1)};
        let gyr_z = U8ArrayWrapper{inner: self._read_16bit_register(Registers::GYR_DATA_X+2)};
        // let gyr_y = self._read_16bit_register(Registers::GYR_DATA_X+1);
        // let gyr_z = self._read_16bit_register(Registers::GYR_DATA_X+2);
        rprintln!("Gyr X: {}, Y: {}, Z: {}", gyr_x.get_u16(), gyr_y.get_u16(), gyr_z.get_u16());
    }

    pub fn read_temp(&mut self) {
        let buf = self._read_16bit_register(Registers::TEMP_DATA);
        let raw_temp = ((buf[1] as i16) << 8) | (buf[0] as i16);
        rprintln!("Temperature: {} degC", (raw_temp/512)+23);
    }

    pub fn soft_reset(&mut self){
        rprintln!("Soft reset started");
        self._write_16bit_register(Registers::CMD, [0xDE,0xAF]);
        rprintln!("Soft reset done");
    }

}

pub trait WriteRead16bit{
    fn _read_16bit_register(&mut self, addr: u8) -> [u8;2];
    fn _write_16bit_register(&mut self, addr: u8, data: [u8;2]);
}

impl WriteRead16bit for Bmi323<SPI1,'A',4>{
    fn _read_16bit_register(&mut self, addr: u8) -> [u8;2]{
        let mut rx = [0x00;4];
        self.cs.set_low();
        self.spi.transfer(&mut rx,&[addr | 0x80,0_u8]).unwrap();
        self.cs.set_high();
        [rx[3],rx[2]]
    }
    fn _write_16bit_register(&mut self, addr: u8, data: [u8;2]){
        let tx = [addr,data[1],data[0]];
        self.cs.set_low();
        self.spi.write(&tx).unwrap();
        self.cs.set_high();
    }
}