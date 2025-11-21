pub mod bmi323;

pub trait Imu{
    fn init(&mut self);
    fn read_register(&mut self,reg: u8)->u8;
    fn write_register(&mut self,reg: u8,value: u8);
    // fn calibrate();
}