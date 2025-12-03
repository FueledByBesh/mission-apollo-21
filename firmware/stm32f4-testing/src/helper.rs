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
    
    
}

#[cfg(test)]
mod test{
    use crate::helper::U8ArrayWrapper;

    #[test]
    fn test_u8_array(){
        let wrapper = U8ArrayWrapper{inner:[0x5]};
        assert_eq!(wrapper.get_u8(),5);
    }
    #[test]
    fn test_u16_array(){
        let wrapper = U8ArrayWrapper{inner:[0x01,0x0]};
        assert_eq!(wrapper.get_u16(),256);
    }
}