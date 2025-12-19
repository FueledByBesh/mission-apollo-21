
pub mod constants;
use constants::Registers;
pub mod config;
use config::{Bmi323Config};

use rtt_target::rprintln;
use stm32f4xx_hal::{pac::SPI1, spi::{Spi, Instance}, gpio::{Pin, Output}, pac};
use crate::helper::{print_binary};


pub struct GyrValue{
    pub gyr_x: f32,
    pub gyr_y: f32,
    pub gyr_z: f32
}

impl GyrValue {
    pub fn update(&mut self, bmi323: &mut Bmi323<pac::SPI1,'A',4>){
        let scale = bmi323.config.unwrap().gyro_config.range.get_lsb_per_dps_value();
        let data = bmi323._continuously_read::<6,8>(Registers::GYR_DATA_X);
        self.gyr_x = i16::from_le_bytes(data[0..2].try_into().unwrap()) as f32 / scale;
        self.gyr_y = i16::from_le_bytes(data[2..4].try_into().unwrap()) as f32 / scale;
        self.gyr_z = i16::from_le_bytes(data[4..6].try_into().unwrap()) as f32 / scale;
    }

    pub fn print(&self){
        rprintln!("Gyr X: {} rad/s, Gyr Y: {} rad/s, Gyr Z: {} rad/s", self.gyr_x,self.gyr_y,self.gyr_z);
    }
}

pub struct Bmi323<SPI,const A: char, const N: u8>
where
    SPI: Instance
{
    spi: Spi<SPI>,
    cs: Pin<A,N,Output>,
    pub config: Option<Bmi323Config>
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
        if buf[0] == 0x43{
            rprintln!("BMI323 detected");
        }else {
            rprintln!("BMI323 not detected"); }
    }

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
        rprintln!("Gyr X: {} rad/s, Gyr Y: {} rad/s, Gyr Z: {} rad/s", gyro_dps[0],gyro_dps[1],gyro_dps[2]);
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
    fn _continuously_read<const LEN:usize, const TOTAL_LEN: usize>(&mut self, start_addr: u8) -> [u8;LEN];
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

    /// [TOTAL_LEN] must be [LEN+2]
    fn _continuously_read<const LEN: usize, const TOTAL_LEN: usize>(&mut self, start_addr: u8) -> [u8;LEN]{
        if LEN<2{ panic!("LEN must be at least 2"); }
        assert_eq!(TOTAL_LEN,LEN+2,"TOTAL_LEN must be LEN+2");
        let tx = [start_addr | 0x80,0_u8];
        let mut rx = [0_u8;TOTAL_LEN];
        self.cs.set_low();
        self.spi.transfer(&mut rx,&tx).unwrap();
        self.cs.set_high();
        rprintln!("{},{},{},{}",rx[0],rx[1],rx[2],rx[3]);
        rx[2..].try_into().unwrap()
    }
}