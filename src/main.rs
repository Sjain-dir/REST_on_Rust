use std::ffi::{
        c_char, 
        c_int,
        c_uint,
        c_void
    };



// struct sockaddr_in {
//     sa_family_t    sin_family; /* address family: AF_INET */
//     in_port_t      sin_port;   /* port in network byte order */
//     struct in_addr sin_addr;   /* internet address */
// };
//
// The <sys/socket.h> header shall define the unsigned integer type sa_family_t.
// typedef uint16_t in_port_t;
// struct in_addr {
//     uint32_t       s_addr;     /* address in network byte order */
// };

struct in_addr {
    s_addr : u32,
}

struct sockaddr_in{
    sin_family : c_uint,
    sin_port   : u16,
    sin_addr   : in_addr,
}

#[link (name = "interface", kind = "static")]
extern "C" {
    // fn test(input : c_int) -> c_char;
    fn socket(domain: c_int, type1 : c_int,protocol : c_int) -> c_int;
    fn htons (__hostshort : u16) -> u16;
    fn inet_pton (__af : c_int, __cp : *mut c_char, __buf : *mut c_void) -> c_int;
    fn connect(__fd : c_int, __addr : *mut sockaddr_in, __len: c_uint) -> c_int;

}

fn main() {
    // let mut output : i8;
    // unsafe {
    //     output = test(5);
    // };
    // let output = (output as u8 ) as char;
    // println!("Hello, world! {:?}", output.to_string());
}
