use stm32f4xx_hal::gpio::{Output, Pin};
use rtt_target::rprintln;


pub fn blink<const P:char, const N: u8>(pin: &mut Pin<P,N,Output>){

    if pin.is_set_low() {
        pin.set_high();
        rprintln!("LED set_high");
    }else {
        pin.set_low();
        rprintln!("LED set_low");
    }

}
