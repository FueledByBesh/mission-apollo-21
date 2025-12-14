use rtt_target::{rprint, rprintln};

pub struct U8ArrayWrapper<const LEN: usize> {
    pub inner: [u8;LEN]
}

impl<const LEN: usize> U8ArrayWrapper<LEN> {
    pub fn get_u8(&self) -> u8{
        if LEN < 1 {
            panic!("Array too small for u8!")
        } else if LEN > 1 {
            panic!("Array doesn't fit into u8!")
        }
        self.inner[0]
    }
    
    pub fn get_u16(&self) -> u16{
        if LEN > 2 {
            panic!("Array doesn't fit into u16!")
        }
        let result = self.inner[0] as u16;
        if LEN == 2 {
            result << 8 | self.inner[1] as u16
        }else {
            result
        }
    }

    // pub fn get_i16(&self) -> i16{
    //     i16::from_be_bytes(self.inner)
    // }
}

pub fn print_binary(byte: u8){
    let x: u8 = byte;
    for i in (0..8).rev() {
        let bit = (x >> i) & 1;
        rprint!("{}", bit);
    }
    rprintln!();
}