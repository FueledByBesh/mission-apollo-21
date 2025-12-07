#![no_std]
#![no_main]

use cortex_m_rt::entry;
// use stm32f1xx_hal::{
//     pac,
//     prelude::*,
//     timer::Timer,
// };

#[entry]
fn main() -> ! {

    loop {
        cortex_m::asm::nop();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
