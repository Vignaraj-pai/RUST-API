use std::any::type_name;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::mem::size_of;
use std::net::SocketAddrV4;
use std::io::Error;
use std::io::ErrorKind;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn create_socket(domain: u32, type_: &str, protocol: u32) -> i64 {

    if type_ == "SOCK_STREAM" {
        return syscalls::sys_socket(domain as u32, 1 as u32, protocol as u32)
    }
    else if type_ == "SOCK_DGRAM" {
        return syscalls::sys_socket(domain as u32, 2 as u32, protocol as u32)
    }
    else {
        // panic
        panic!("Invalid socket type: {}", type_); 
    }
}

fn bind_socket(sockfd: u64, addr: &std::net::SocketAddr, addrlen: u32) -> i64 {
    return syscalls::sys_bind(sockfd, addr, addrlen);
}

fn listen_socket(sockfd: u64, backlog: u32) -> i64 {
    return syscalls::sys_listen(sockfd, backlog);
}

fn accept_socket(sockfd: u64, addr: &std::net::SocketAddr, addrlen: &u32) -> i64 {
    return syscalls::sys_accept(sockfd, addr, addrlen);
}

fn connect_socket(sockfd: u64, addr: &std::net::SocketAddr, addrlen: u32) -> i64 {
    return syscalls::sys_connect(sockfd, addr, addrlen);
}

fn sendto_socket(sockfd: u64, buf: &str, len: u64, flags: u32, dest_addr: &str, addrlen: u32) -> i64 {
    let buf = buf.as_bytes();
    let dest_addr = dest_addr.as_bytes();
    return syscalls::sys_sendto(sockfd, buf.as_ptr(), len, flags, dest_addr.as_ptr(), addrlen);
}

fn recvfrom_socket(sockfd: u64, buf: &mut [u8], len: u64, flags: u32, src_addr: *mut std::net::SocketAddr, addrlen: *mut u32) -> i64 {
    let buf_ptr = buf.as_mut_ptr();
    let src_addr_ptr = src_addr as *mut u8;
    return syscalls::sys_recvfrom(sockfd, buf_ptr, len, flags, src_addr_ptr, addrlen);
}


trait OInput1<T>
{
    fn o_take_inp_1(&self, t: T);
}

trait OInput2<T, U, V>
{
    fn o_take_inp_2(&self, t: T, u: U, v: V);
}

struct Input;
impl Input
{
    fn take_inp_1<T>(&self, t: T) where Self: OInput1<T>
    {
        self.o_take_inp_1(t);
    }

    fn take_inp_2<T, U, V>(&self, t: T, u: U, v: V) where Self: OInput2<T, U, V>
    {
        self.o_take_inp_2(t, u, v);
    }
}

impl OInput1<&mut u64> for Input
{
    fn o_take_inp_1(&self, t: &mut u64)
    {
        let mut inp: u64 = 0;
        let mut num: u64;
        let mut c: u8 = 0;
        loop {
            syscalls::sys_read(0, &mut c, 1);
            if c == 0 {
                break;
            }
            if c == 10 {
                break;
            }
            if c >= 48 && c <= 57 {
                num = (c - 48).into();
                inp = inp * 10 + num;
            } else {
                //panic if there is a non-number character
                panic!("Invalid input");
            }
        }
        *t = inp;
    }
}

impl OInput1<&mut i64> for Input
{
    fn o_take_inp_1(&self, t: &mut i64)
    {
        let mut inp: i64 = 0;
        let mut num: i64;
        let mut c: u8 = 0;
        let mut neg: bool = false;
        loop {
            syscalls::sys_read(0, &mut c, 1);
            if c == 0 {
                break;
            }
            if c == 10 {
                break;
            }
            if!neg && c == 45 {
                neg = true;
                continue;
            }
            if c >= 48 && c <= 57 {
                num = (c - 48).into();
                inp = inp * 10 + num;
            } else {
                //panic if there is a non-number character
                panic!("Invalid input");
            }
        }
        if neg {
            *t = -inp;
        }
        else {
            *t = inp;
        }
    }
}

impl OInput2<&mut String, i64, char> for Input
{
    fn o_take_inp_2(&self, t: &mut String, u: i64, v: char) {
        t.clear();
        let mut c: u8 = 0;
        let mut i: i64 = 0;
        let mut ch: char;
        loop {
            syscalls::sys_read(0, &mut c, 1);
            ch = c as char;
            if u != -1 && i >= u {
                break;
            }
            if ch == v {
                break;
            }
            t.push(ch);
            i = i + 1;
        }
    }
}

macro_rules! input {
    ($arg:expr) => {
        Input.take_inp_1($arg);
    };

    ($arg1:expr, $arg2:expr, $arg3:expr) => {
        Input.take_inp_2($arg1, $arg2, $arg3);
    }
}

macro_rules! output {
    ($($arg:tt)*) => {
        $(  
            syscalls::sys_write(1, $arg.as_bytes().as_ptr(), $arg.len().try_into().unwrap());
        )*
    };
}





fn main() {

    let server_fd: i64 = create_socket(2, "SOCK_STREAM", 0);

    if server_fd < 0 {
        output!("Socket creation failed\n");
        return;
    } else {
        output!("Socket creation successful\n");
    }
      
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8080;
    
    let mut socket_addr = SocketAddr::V4(SocketAddrV4::new(ip_addr, port));

    let mut addr_len = std::mem::size_of::<SocketAddr>() as u32;
    
    let bs = bind_socket(server_fd as u64, &socket_addr, 16);

    if listen_socket(server_fd as u64, 16) < 0 {
        output!("Listen failed\n");
        return;
    } else {
        output!("Listen successful\n");
    }

    let new_socket = accept_socket(server_fd as u64, &mut socket_addr, &addr_len);

    println!("{}", new_socket);

    if new_socket == -1 {
        output!("Accept failed\n");
        return
    } else {
        output!("Accept successful\n");
    }
}
