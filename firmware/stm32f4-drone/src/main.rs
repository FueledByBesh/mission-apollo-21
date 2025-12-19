#![no_std]
#![no_main]
mod drivers;
mod app;
mod helper;
mod pid; // simple PID controller module

use cortex_m::register::msp::read;
use cortex_m_rt::{entry};
use rtt_target::{rtt_init_print,rprintln};
use stm32f4xx_hal::{prelude::_fugit_RateExtU32, pac, pac::interrupt};

use app::{App};
use crate::drivers::{
    nrf24l01::{
        commands::Commands
    }
};

// #[allow(static_mut_refs)]
// #[interrupt]
// fn TIM2(){
//     unsafe {
//         // 1. Сброс флага
//         (*pac::TIM2::ptr()).sr().modify(|_, w| w.uif().clear_bit());
//
//         static mut COUNTER: usize = 0;
//         COUNTER+=1;
//         let a = COUNTER.clone();
//         rprintln!("TIM2 interrupt {}",a)
//     }
// }

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("App going to start");
    let mut app = App::init();
    rprintln!("App started");
    let mut delay = app.delay;
    let mut radio = app.modules.radio.expect("Radio not found");

    radio.min_setup();
    delay.delay_ms(20);
    radio.read_rx();

    loop {
        delay.delay_ms(200);
        if !radio.is_rx_fifo_empty(){
            rprintln!("RX FIFO not empty");
            let a = radio.read_payload();
            rprintln!("PAYLOAD: {:?}",a);
        }else { rprintln!("RX FIFO empty") }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC: {}!", info);
    loop {}
}





