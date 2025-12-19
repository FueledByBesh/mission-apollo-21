use rtt_target::{rprint, rprintln};

pub fn print_array(array: &[u8]){
    for byte in array {
        rprint!("{} ",byte);
    }
    rprintln!()
}