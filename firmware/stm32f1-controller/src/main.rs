#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
// use stm32f1xx_hal::{
//     pac,
//     prelude::*,
//     timer::Timer,
// };

#[entry]
fn main() -> ! {

    loop {
        // CPU ничего не делает
    }
}
