#![no_std]
#![no_main]
mod drivers;
mod app;
mod helper;

use core::ptr::read;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use stm32f1xx_hal::{
    prelude::*
};
use stm32f1xx_hal::hal_02::digital::v2::InputPin;
use crate::app::App;
use crate::drivers::nrf24l01::{Commands, WriteRead};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut app = App::init();
    let mut delay = &mut app.delay;
    let mut joysticks = app.modules.joysticks;

    let mut led = app.modules.led;
    let mut radio = app.modules.radio;
    radio.min_setup();
    delay.delay_ms(1000);
    radio.read_tx_address();
    loop {
        delay.delay_ms(100);
        joysticks.update();
        // joysticks.print();
        if joysticks.left.2 {
            rprintln!("Left Button Pressed!");
            radio.stop_tx();
            if radio.is_tx_fifo_full() {
                rprintln!("TX FIFO FULL!");
            }else {
                radio.write_payload(&[0x01, 0x02, 0x03, 0x04, 0x05]);
                rprintln!("TX PAYLOAD SENT!");
            }

        }
        if joysticks.right.2 {
            rprintln!("Right Button Pressed!");
            radio.send_tx(&mut delay);
            if radio.is_data_sent() {
                rprintln!("TX Send!");
                led.toggle();
            }else {
                rprintln!("TX NOT SENT!");
            }
            radio.observe_tx();
        }
    }
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC: {}!", info);
    loop {}
}
