use rtt_target::rprintln;
use stm32f4xx_hal::pac::TIM3;
use stm32f4xx_hal::timer::CPin;
use stm32f4xx_hal::timer::pwm::PwmChannel;

// pub struct Esc{
//     left_front: Motor<TIM3,0>,
//     right_front: Motor<TIM3,1>,
//     left_back: Motor<TIM3,2>,
//     right_back: Motor<TIM3,3>,
// }

// pub struct Motor<TIM,const C: u8> {
//     pub channel: PwmChannel<TIM,C>
// }

pub struct Motor<const C: u8> {
    pub channel: PwmChannel<TIM3,0>
}

impl Motor<0> {

    pub fn set_duty(&mut self, duty: u8){
        let max_duty = self.channel.get_max_duty();
        let duty = max_duty as f32/(100_f32/duty as f32);
        rprintln!("Duty in set_duty function: {}",duty as u16);
        self.channel.set_duty(duty as u16);
    }

}