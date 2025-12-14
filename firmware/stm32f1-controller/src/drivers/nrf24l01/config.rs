// use stm32f1xx_hal::hal::digital::OutputPin;
// use stm32f1xx_hal::pac::SPI1;
// use crate::drivers::nrf24l01::{Nrf24l01, WriteRead8bit};
// use crate::drivers::nrf24l01::registers::Registers;
// 
// pub enum Mode{
//     Receiver,
//     Transmitter
// }
// 
// pub trait Configure{
//     fn power_up(&mut self);
//     fn power_down(&mut self);
//     fn set_mode(&mut self, mode: Mode);
//     fn enable_pipe(&mut self, pipe: u8,pipe_address: [u8;5]);
//     fn enable_crc(&mut self,size_2byte: bool);
// }
// 
// impl<PIN> Configure for Nrf24l01<SPI1, u8, PIN>
// where PIN: OutputPin
// {
//     fn power_up(&mut self) {
//         let buf = self.read_8bit(Registers::CONFIG);
//         self.write_8bit(Registers::CONFIG,buf | 0x01<<1);
//     }
// 
//     fn power_down(&mut self) {
//         let buf = self.read_8bit(Registers::CONFIG);
//         self.write_8bit(Registers::CONFIG,buf & !(0x01<<1));
//     }
// 
//     fn set_mode(&mut self, mode: Mode) {
//         let buf = self.read_8bit(Registers::CONFIG);
//         match mode {
//             Mode::Receiver => {
//                 self.write_8bit(Registers::CONFIG,buf | (0x01));
//             }
//             Mode::Transmitter => {
//                 self.write_8bit(Registers::CONFIG,buf & !(0x01));
//             }
//         }
//     }
// 
//     fn enable_crc(&mut self,size_2byte: bool){
//         let write: u8;
//         if size_2byte{
//             write = 0b11<<2;
//         }else {
//             write = 0b10<<2;
//         }
//         let mut buf = self.read_8bit(Registers::EN_AA);
//         buf &= !(0b11<<2);
//         self.write_8bit(Registers::EN_AA,buf | write);
//     }
// 
//     fn enable_pipe(&mut self, pipe: u8, pipe_address: [u8; 5]) {
//         todo!()
//     }
// 
// }