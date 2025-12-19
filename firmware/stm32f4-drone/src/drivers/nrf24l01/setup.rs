use super::{Nrf24l01,WriteRead};
use super::registers::Registers;

impl Nrf24l01{
    pub fn min_setup(&mut self){
        self.config_register();
        self.en_aa_register();
        self.en_rxaddr_register();
        self.setup_aw_register();
        self.setup_retr_register();
        // self.rx_addr_p0();
    }

    fn config_register(&mut self){
        let write:u8 = 0b0111_1111;
        self.write_8bit(Registers::CONFIG,write)
    }
    fn en_aa_register(&mut self){
        let write:u8 = 0b0000_0000;
        self.write_8bit(Registers::EN_AA,write)
    }
    fn en_rxaddr_register(&mut self){
        let write:u8 = 0b0000_0001;
        self.write_8bit(Registers::EN_RXADDR,write)
    }
    fn setup_aw_register(&mut self){
        let write:u8 = 0b11;
        self.write_8bit(Registers::SETUP_AW,write)
    }

    fn setup_retr_register(&mut self){
        self.write_8bit(Registers::SETUP_RETR,0)
    }

    fn rx_addr_p0(&mut self) {
        let write: [u8; 5] = [0x24, 0x24, 0x24, 0x24, 0x24];
        self.write_40bit(Registers::RX_ADDR_P0, write)
    }

    // fn tx_addr(&mut self) {
    //     let write: [u8; 5] = [0x24, 0x24, 0x24, 0x24, 0x24];
    //     self.write_40bit(Registers::TX_ADDR, write)
    // }
}