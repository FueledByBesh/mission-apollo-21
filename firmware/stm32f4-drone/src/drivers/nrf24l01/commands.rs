use cortex_m::delay::Delay;
use rtt_target::{rprintln};
use super::{
    Nrf24l01,
    WriteRead,
    registers::Registers
};
use crate::helper::{print_array, print_binary};

pub trait Commands{
    fn power_up(&mut self);
    fn power_down(&mut self);
    fn read_payload(&mut self);
    fn is_rx_fifo_empty(&mut self) -> bool;
    fn flush_rx(&mut self);

    fn read_rx(&mut self);
    fn stop_read_rx(&mut self);
}

impl Commands for Nrf24l01{
    fn power_up(&mut self) {
        todo!()
    }

    fn power_down(&mut self) {
        todo!()
    }

    fn read_payload(&mut self) {
        let mut rx: [u8; 32] = [0; 32];
        self.csn.set_low();
        self.spi.write(&[0b0110_0001]).unwrap();
        self.spi.read(&mut rx).unwrap();
        self.csn.set_high();
        print_array(&rx)
    }

    fn is_rx_fifo_empty(&mut self) -> bool {
        let a = self.read_8bit(Registers::FIFO_STATUS);
        // rprintln!("STATUS: {}",a);
        (a & 0b0000_0001) == 0b0000_0001
    }

    fn flush_rx(&mut self) {
        todo!()
    }

    fn read_rx(&mut self) {
        self.ce.set_high()
    }
    fn stop_read_rx(&mut self) {
        self.ce.set_low()
    }
}