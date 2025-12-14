mod registers;
mod config;
mod enums;

use stm32f1xx_hal::gpio::Pin;
use stm32f1xx_hal::hal::digital::OutputPin;
use stm32f1xx_hal::pac::SPI1;
use stm32f1xx_hal::spi::{Instance, Spi};
use crate::DelayWrapper;
use crate::drivers::nrf24l01::registers::Registers;

pub struct Nrf24l01<SPI, W, PIN1,PIN2>
where
    SPI: Instance,
    PIN1: OutputPin,
    PIN2: OutputPin
{
    spi: Spi<SPI, W>,
    csn: PIN1,
    ce: PIN2
}

impl<SPI, W, PIN1,PIN2> Nrf24l01<SPI, W, PIN1,PIN2>
where
    SPI: Instance,
    PIN1: OutputPin,
    PIN2: OutputPin
{
    pub fn new(spi: Spi<SPI, W>, csn: PIN1, ce: PIN2) -> Self {
        Self { spi, csn, ce }
    }
}

pub trait WriteRead{
    fn read_8bit(&mut self, addr: u8) -> u8;
    fn write_8bit(&mut self, addr: u8,data: u8);
    fn write_40bit(&mut self, addr: u8, data: [u8; 5]);
    fn write_payload(&mut self, payload: &[u8]);
}

impl<PIN1,PIN2> WriteRead for Nrf24l01<SPI1, u8, PIN1,PIN2>
where
    PIN1: OutputPin,
    PIN2: OutputPin
{
    fn read_8bit(&mut self, addr: u8) -> u8 {
        let mut rx = [0u8; 2];
        self.csn.set_low().unwrap();
        self.spi.transfer(&mut rx,&[addr,0_u8]).unwrap();
        self.csn.set_high().unwrap();
        rx[1]
    }

    fn write_8bit(&mut self, addr: u8, data: u8) {
        self.csn.set_low().unwrap();
        self.spi.write(&[addr | (1<<5),data]).unwrap();
        self.csn.set_high().unwrap();
    }
    ///[[msb, ... ,lsb]] format
    fn write_40bit(&mut self, addr: u8, data: [u8; 5]){
        self.csn.set_low().unwrap();
        self.spi.write(&[addr,data[4],data[3],data[2],data[1],data[0]]).unwrap();
        self.csn.set_high().unwrap();
    }

    fn write_payload(&mut self, payload: &[u8]){
        self.csn.set_low().unwrap();
        self.spi.write(&[0b1010_0000]).unwrap();
        self.spi.write(payload).unwrap();
        self.csn.set_high().unwrap();
    }
}




impl<PIN1,PIN2> Nrf24l01<SPI1,u8,PIN1,PIN2>
where
    PIN1: OutputPin,
    PIN2: OutputPin
{

    pub fn send(&mut self, delay: &mut DelayWrapper){
        self.ce.set_high().unwrap();
        delay.delay.delay_us(15);
        self.ce.set_low().unwrap();
    }

    pub fn min_setup(&mut self){
        self.config_register();
        self.en_aa_register();
        self.en_rxaddr_register();
        self.setup_aw_register();
        self.setup_retr_register();
        self.tx_addr();
    }

    fn config_register(&mut self){
        let write:u8 = 0b0111_1110;
        self.write_8bit(Registers::CONFIG,write)
    }
    fn en_aa_register(&mut self){
        let write:u8 = 0b0000_0000;
        self.write_8bit(Registers::EN_AA,write)
    }
    fn en_rxaddr_register(&mut self){
        let write:u8 = 0b0000_0000;
        self.write_8bit(Registers::EN_RXADDR,write)
    }
    fn setup_aw_register(&mut self){
        let write:u8 = 0b11;
        self.write_8bit(Registers::SETUP_AW,write)
    }

    fn setup_retr_register(&mut self){
        self.write_8bit(Registers::SETUP_RETR,0)
    }

    // fn rx_addr_p0(&mut self) {
    //     let write: [u8; 5] = [0x15, 0x15, 0x15, 0x15, 0x15];
    //     self.write_40bit(Registers::RX_ADDR_P0, write)
    // }

    fn tx_addr(&mut self) {
        let write: [u8; 5] = [0x24, 0x24, 0x24, 0x24, 0x24];
        self.write_40bit(Registers::TX_ADDR, write)
    }
}