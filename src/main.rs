use std::os::raw::{
        c_char, 
        c_int
    };
use std::char;


#[link (name = "interface", kind = "static")]
extern "C" {
    fn test(input : c_int) -> c_char;
}

fn main() {
    let mut output : i8;
    unsafe {
        output = test(5);
    };
    let output = (output as u8 ) as char;
    println!("Hello, world! {:?}", output.to_string());
}
