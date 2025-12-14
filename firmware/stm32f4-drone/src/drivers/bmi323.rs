
pub mod constants;
use constants::Registers;
pub mod config;
use config::{Bmi323Config};

use rtt_target::rprintln;
use stm32f4xx_hal::{
    pac::SPI1,
    spi::{Spi,Instance},
    gpio::{Pin, Output}
};
use crate::helper::{U8ArrayWrapper,print_binary};

pub struct Bmi323<SPI,const A: char, const N: u8>
where
    SPI: Instance
{
    spi: Spi<SPI>,
    cs: Pin<A,N,Output>,
    config: Option<Bmi323Config>
}

impl<SPI,const A:char,const N:u8> Bmi323<SPI,A,N>
where
    SPI: Instance
{
    pub fn new(spi: Spi<SPI>, cs: Pin<A,N,Output>) -> Self{
        Self{ spi,cs, config: None }
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

    // pub fn read_raw_gyr(&mut self){
    //     let gyr_x = U8ArrayWrapper{inner: self._read_16bit_register(Registers::GYR_DATA_X)};
    //     let gyr_y = U8ArrayWrapper{inner: self._read_16bit_register(Registers::GYR_DATA_X+1)};
    //     let gyr_z = U8ArrayWrapper{inner: self._read_16bit_register(Registers::GYR_DATA_X+2)};
    //     // let gyr_y = self._read_16bit_register(Registers::GYR_DATA_X+1);
    //     // let gyr_z = self._read_16bit_register(Registers::GYR_DATA_X+2);
    //     rprintln!("Gyr X: {}, Y: {}, Z: {}", gyr_x.get_u16(), gyr_y.get_u16(), gyr_z.get_u16());
    // }

    pub fn read_gyr(&mut self){
        let gx = self._read_16bit_register(Registers::GYR_DATA_X);
        let gy = self._read_16bit_register(Registers::GYR_DATA_X+1);
        let gz = self._read_16bit_register(Registers::GYR_DATA_X+2);

        let scale = self.config.unwrap().gyro_config.range.get_lsb_per_dps_value();

        let gyro_dps = [
            i16::from_le_bytes(gx) as f32 / scale,
            i16::from_le_bytes(gy) as f32 / scale,
            i16::from_le_bytes(gz) as f32 / scale,
        ];
        rprintln!("Gyr X: {} rad/s, Gyr Y: {} rad/s, Gyr Y: {} rad/s", gyro_dps[0],gyro_dps[1],gyro_dps[2]);
        // rprintln!("Gyr Y: {} rad/s", gyro_dps[1]);
        // rprintln!("Gyr Y: {} rad/s", gyro_dps[2]);
    }

    pub fn read_temp(&mut self) {
        let buf = self._read_16bit_register(Registers::TEMP_DATA);
        let raw_temp = i16::from_le_bytes(buf);
        let temp_c = raw_temp as f32 / 512.0 + 23.0;
        rprintln!("Temperature: {} degC", temp_c);
    }

    pub fn get_error(&mut self){
        let buf = self._read_16bit_register(Registers::ERR_REG);
        // rprintln!("Error register: {:08b} {08b}", buf[1], buf[0]);
        print_binary(buf[0]);
        let fatal_err: bool = buf[0] & 1 == 1;
        rprintln!("Fatal error: {}", fatal_err);
        let acc_conf_err: bool = (buf[0]>>5) & 1 == 1;
        let gyr_conf_err: bool = (buf[0]>>6) & 1 == 1;
        rprintln!("Acc config error: {}", acc_conf_err);
        rprintln!("Gyr config error: {}", gyr_conf_err);
    }

    pub fn soft_reset(&mut self){
        rprintln!("Soft reset started");
        self._write_16bit_register(Registers::CMD, [0xDE,0xAF]);
        rprintln!("Soft reset done");
        self.config = None;
    }

}

pub trait WriteRead16bit{
    fn _read_16bit_register(&mut self, addr: u8) -> [u8;2];
    fn _write_16bit_register(&mut self, addr: u8, data: [u8;2]);
}

impl WriteRead16bit for Bmi323<SPI1,'A',4>{

    /// returns [lsb,msb]
    fn _read_16bit_register(&mut self, addr: u8) -> [u8;2]{
        let mut rx = [0x00;4];
        self.cs.set_low();
        self.spi.transfer(&mut rx,&[addr | 0x80,0_u8]).unwrap();
        self.cs.set_high();
        [rx[2],rx[3]]
    }
    /// data is [msb,lsb]
    fn _write_16bit_register(&mut self, addr: u8, data: [u8;2]){
        let tx = [addr,data[1],data[0]];
        self.cs.set_low();
        self.spi.write(&tx).unwrap();
        self.cs.set_high();
    }
}