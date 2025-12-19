use rtt_target::rprintln;
use stm32f1xx_hal::adc::{Adc, SampleTime};
use stm32f1xx_hal::gpio::{Pin, Analog, Input, PullUp};
use stm32f1xx_hal::pac::{ADC1};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::rcc::Rcc;

pub struct Joysticks {
    pub left: Joystick,
    pub right: Joystick,
    left_x_pin: Pin<'B',0,Analog>,
    left_y_pin: Pin<'B', 1,Analog>,
    left_sw: Pin<'B',10,Input<PullUp>>,
    right_x_pin: Pin<'A',0,Analog>,
    right_y_pin: Pin<'A', 1,Analog>,
    right_sw: Pin<'C',15,Input<PullUp>>,
    adc: Adc<ADC1>
}

impl Joysticks {

    pub fn init(
        adc1: ADC1,
        left_x_pin: Pin<'B',0,Analog>,
        left_y_pin: Pin<'B',1,Analog>,
        left_sw: Pin<'B',10,Input<PullUp>>,
        right_x_pin: Pin<'A', 0,Analog>,
        right_y_pin: Pin<'A', 1,Analog>,
        right_sw: Pin<'C',15,Input<PullUp>>,
        rcc: &mut Rcc
    ) -> Self{
        let mut adc = Adc::new(adc1,rcc);
        adc.set_sample_time(SampleTime::T_55);
        Self{
            left: Joystick(0,0,false),
            right: Joystick(0,0,false),
            left_x_pin,
            left_y_pin,
            left_sw,
            right_x_pin,
            right_y_pin,
            right_sw,
            adc
        }
    }

    pub fn update(&mut self){
        let left_x: u16 = self.adc.read(&mut self.left_x_pin).unwrap();
        let left_y: u16 = self.adc.read(&mut self.left_y_pin).unwrap();
        let left_sw = self.left_sw.is_low();
        let right_x: u16 = self.adc.read(&mut self.right_x_pin).unwrap();
        let right_y: u16 = self.adc.read(&mut self.right_y_pin).unwrap();
        let right_sw = self.right_sw.is_low();
        self.left = Joystick(left_x,left_y,left_sw);
        self.right = Joystick(right_x,right_y,right_sw);
    }

    pub fn print(&self){
        rprintln!("Left: ({},{},{}), Right: ({},{},{})",self.left.0,self.left.1,self.left.2,self.right.0,self.right.1,self.right.2)
    }

}

pub struct Joystick(pub u16,pub u16,pub bool);

