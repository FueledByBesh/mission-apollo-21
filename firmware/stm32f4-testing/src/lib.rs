mod helper;

#[cfg(test)]
extern crate std;


#[cfg(test)]
mod tests {
    use crate::helper::U8ArrayWrapper;

    #[test]
    fn test_u8(){
        let wrapper = U8ArrayWrapper{inner:[0x55]};
        assert_eq!(wrapper.get_u8(), 85);
        assert_eq!(wrapper.get_u16(), 85);
    }

    #[test]
    fn test_u16(){
        let wrapper = U8ArrayWrapper{inner:[0x55, 0xAA,5]};
        assert_eq!(wrapper.get_u16(), 21930);
    }
}