use stm32f4xx_hal::{hal,hal_02};
use hal_02::blocking::spi::Transfer;
use hal_02::digital::v2::OutputPin;
use super::*;


pub struct Bmi323<'a, SPI,CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin
{
    spi: &'a mut SPI,
    cs: &'a mut CS
}

impl<'a ,SPI,CS> Bmi323<'a, SPI,CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin
{
    pub fn new(spi: &'a mut SPI, cs: &'a mut CS) -> Self {
        Self{spi,cs}
    }
}

impl<'a ,SPI,CS> Imu for Bmi323<'a, SPI,CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin
{
    fn init(&mut self) {
        self.write_register( 0x7E, 0xB6);
        cortex_m::asm::delay(10_000_0);

        // accel enable
        self.write_register(0x40, 0x04);

        // gyro enable
        self.write_register(0x42, 0x04);

        // ODR accel = 100 Hz
        self.write_register(0x41, 0x0A);

        // ODR gyro = 100 Hz
        self.write_register(0x43, 0x0A);
    }

    fn read_register(&mut self, reg: u8) -> u8 {
        // let addr = reg | 0x80; // set MSB for read
        let mut buf = [reg | 0x80, 0x00];

        self.cs.set_low().ok();
        self.spi.transfer(&mut buf).ok();
        self.cs.set_high().ok();

        buf[1]
    }

    fn write_register(&mut self, reg: u8, value: u8) {
        let addr = reg & 0x7F; // clear MSB for write

        self.cs.set_low().ok();
        self.spi.transfer(&mut [addr, value]).ok();
        self.cs.set_high().ok();
    }
}
