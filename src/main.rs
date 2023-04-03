use std::ffi::{
        c_char, 
        c_int,
        c_uint,
        c_void, 
        c_ulong, 
        c_long,
        CString
    };
    
    
    #[link (name = "interface", kind = "static")]
    extern "C" {
        // fn test(input : c_int) -> c_char;
        fn socket(domain: c_int, type1 : c_int,protocol : c_int) -> c_int;
        fn htons(__hostshort : u16) -> u16;
        fn inet_pton(__af : c_int, __cp : *mut c_char, __buf : *mut c_void) -> c_int;
        fn connect(__fd : c_int, __addr : *mut sockaddr_in, __len: c_uint) -> c_int;
        fn send(__fd : c_int, __buf : *mut c_void, __n : c_ulong, __flag : c_int ) -> c_long;
        //------------test_edition----------- 
        fn test_interface(domain : *const c_char, port : *mut c_int, payload : *const c_char) -> c_int;
        fn test_fun(i : *mut c_int);
        fn test_retry(domain : *const c_char, i : *mut c_int, payload :*const c_char) -> c_int;
    }
    
    fn main() {
        unsafe {
            let mut payload = CString::new("Likhe jo khat tujhe, jo teri yaad mee...........").expect("lol error in payload");
            let mut port: c_int = 8080;
            let mut domain = CString::new("127.0.0.1").expect("lol error in domain");
    
            
            test_fun(&mut port);
            test_retry(domain.as_ptr(), &mut port ,payload.as_ptr());
            test_interface(domain.as_ptr(), &mut port, payload.as_ptr());
        }
    }
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

const PF_INET : c_int = 2;
const AF_INET : c_int = PF_INET;


//#[derive(Debug)]

struct in_addr {
    s_addr : u32,
}

//#[derive(Debug)]
struct sockaddr_in {
    sin_family : c_uint,
    sin_port   : u16,
    sin_addr   : in_addr,
}



// fn main() {
//     // let mut output : i8;
//     // unsafe {
//     //     output = test(5);
//     // };
//     // let output = (output as u8 ) as char;
//     // println!("Hello, world! {:?}", output.to_string());
//     test()
// }
/*
todo list :
1. test the struct sockaddr_in, like whether buffer is necessary or not
2. do something of type conversion, like usize to long 
3. make use of function inet_pton()
4. send a request
*/

fn test() {
    //let payload = CString::from("likhe jo khat tujhe, jo teri yaad me.....");
    
    unsafe {
        let payload = CString::new("Likhe jo khat tujhe, jo teri yaad mee...........").expect("lol error in payload");
        let mut port: c_int = 8080;
        let domain = CString::new("127.0.0.1").expect("lol error in domain");

        
        test_fun(&mut port);
       // test_interface(domain, &mut port, payload);
        // let client_fd = socket(2, 1, 0);
        // let serv_addr = sockaddr_in {
        //     sin_family : 2,
        //     sin_port   : htons(8080),
        //     sin_addr   : in_addr { s_addr: 16777343 },
        // };
        // println!("{:?}", mem::size_of::<sockaddr_in>());
        // println!("{:?}", client_fd);
        // println!("{:?}", serv_addr);
        //let status = connect(client_fd, serv_addr, mem::size_of::<sockaddr_in>() as u32);
        //let status = connect(client_fd, &serv_addr, mem::size_of::<sockaddr_in>() as u32);

        //send(client_fd, payload, payload.len(), 0);
        // note to me in future, may be convert every datatype according to rust for better compatiblity or idk may be a fucnction which can help converitng values, or may be some thing else idk, plz help me 🥲
        
        
        //-------------test edition-------------------

        // let test_string= payload as i8;
        //let *mut pay : c_char = 

    }
}
