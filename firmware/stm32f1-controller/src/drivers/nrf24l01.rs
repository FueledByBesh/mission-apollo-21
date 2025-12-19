mod registers;
mod config;
mod enums;

use cortex_m::delay::Delay;
use rtt_target::rprintln;
use stm32f1xx_hal::gpio::{Output, Pin, PushPull};
use stm32f1xx_hal::pac::{SPI1};
use stm32f1xx_hal::spi::{Instance, Spi};
use crate::drivers::nrf24l01::registers::Registers;
use crate::helper::print_array;
pub struct Nrf24l01
{
    spi: Spi<SPI1,u8>,
    csn: Pin<'A',11,Output<PushPull>>,
    ce: Pin<'A',10,Output<PushPull>>
}

impl Nrf24l01
{
    pub fn new(spi: Spi<SPI1,u8>, ce: Pin<'A',10,Output<PushPull>>, csn: Pin<'A',11,Output<PushPull>>) -> Self {
        Self { spi, csn, ce }
    }
}

pub trait WriteRead{
    fn read_8bit(&mut self, addr: u8) -> u8;
    fn write_8bit(&mut self, addr: u8,data: u8);
    fn write_40bit(&mut self, addr: u8, data: [u8; 5]);
    fn read_40bit(&mut self, addr: u8) ;
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

    fn read_40bit(&mut self, addr: u8)  {
        let mut rx = [0u8; 6];
        self.csn.set_low();
        self.spi.transfer(&mut rx,&[addr]).unwrap();
        self.csn.set_high();
        print_array(&rx[1..6])
    }
}

pub trait Commands{
    fn power_up(&mut self);
    fn power_down(&mut self);
    fn write_payload(&mut self, payload: &[u8]);
    fn is_tx_fifo_full(&mut self) -> bool;
    fn flush_tx(&mut self);

    fn send_tx(&mut self,delay: &mut Delay);
    fn stop_tx(&mut self);
    fn is_data_sent(&mut self) -> bool;
    fn observe_tx(&mut self);
}

impl Commands for Nrf24l01 {

    fn power_up(&mut self) {
        todo!()
    }

    fn power_down(&mut self) {
        todo!()
    }

    fn write_payload(&mut self, payload: &[u8]) {
        self.csn.set_low();
        self.spi.write(&[0b1010_0000]).unwrap();
        self.spi.write(payload).unwrap();
        self.csn.set_high();
    }

    fn is_tx_fifo_full(&mut self) -> bool {
        let a = self.read_8bit(Registers::STATUS);
        rprintln!("STATUS: {}",a);
        (a & 0b0000_0001) == 0b0000_0001
    }

    fn flush_tx(&mut self){
        self.csn.set_low();
        self.spi.write(&[0b_1110_0001]).unwrap();
        self.csn.set_high();
    }

    fn send_tx(&mut self,delay: &mut Delay) {
        self.ce.set_high();
        // delay.delay_us();
        // self.ce.set_low();
    }
    fn stop_tx(&mut self){
        self.ce.set_low();
    }

    fn is_data_sent(&mut self) -> bool {
        let res = self.read_8bit(Registers::STATUS);
        let result = (res & 1<<5) == 1<<5;
        if result {
            self.write_8bit(Registers::STATUS, 0)
        }
        result
    }

    fn observe_tx(&mut self) {
        let res = self.read_8bit(0x08);
        let loss = (res & 0b1111_1000)>>3;
        let count = res & 0b0000_0111;
        rprintln!("Loss count: {}, Retransmit Count: {}",loss, count);
    }
}




impl Nrf24l01 {

    pub fn read_tx_address(&mut self){
        self.read_40bit(Registers::TX_ADDR);
    }


    pub fn min_setup(&mut self){
        self.config_register();
        self.en_aa_register();
        self.en_rxaddr_register();
        self.setup_aw_register();
        self.setup_retr_register();
        // self.tx_addr();
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