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

trait OOutput1<T>
{
    fn o_put_1(&self, t: T);
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

struct Output;
impl Output
{
    fn put_1<T>(&self, t: T) where Self: OOutput1<T>
    {
        self.o_put_1(t);
    }
}

impl OOutput1<&str> for Output
{
    fn o_put_1(&self, t: &str)
    {
        syscalls::sys_write(1, t.as_bytes().as_ptr(), t.len().try_into().unwrap());
    }
}

impl OOutput1<&String> for Output
{
    fn o_put_1(&self, t: &String)
    {
        syscalls::sys_write(1, t.as_bytes().as_ptr(), t.len().try_into().unwrap());
    }
}

impl OOutput1<String> for Output
{
    fn o_put_1(&self, t: String)
    {
        syscalls::sys_write(1, t.as_bytes().as_ptr(), t.len().try_into().unwrap());
    }
}

impl OOutput1<&char> for Output
{
    fn o_put_1(&self, t: &char)
    {
        let c: u8 = *t as u8;
        syscalls::sys_write(1, &c, 1);
    }
}

impl OOutput1<&u8> for Output
{
    fn o_put_1(&self, t: &u8)
    {
        syscalls::sys_write(1, t, 1);
    }
}

impl OOutput1<&i8> for Output
{
    fn o_put_1(&self, t: &i8)
    {
        syscalls::sys_write(1, t as *const i8 as *const u8, 1);
    }
}

impl OOutput1<u16> for Output
{
    fn o_put_1(&self, t: u16)
    {
        let mut num: u16 = t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        if num == 0 {
            arr[0] = 48;
            i = 1;
        }
        while num > 0 {
            arr[i as usize] = (num % 10) as u8 + 48;
            num = num / 10;
            i = i + 1;
        }
        j = i - 1;
        while j >= 0 {
            syscalls::sys_write(1, &arr[j as usize], 1);
            j = j - 1;
        }
    }
}

impl OOutput1<i16> for Output
{
    fn o_put_1(&self, t: i16)
    {
        let mut num: i16 = t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        let mut neg: bool = false;
        if num == 0 {
            arr[0] = 48;
            i = 1;
        }
        if num < 0 {
            neg = true;
            num = -num;
        }
        while num > 0 {
            arr[i as usize] = (num % 10) as u8 + 48;
            num = num / 10;
            i = i + 1;
        }
        if neg {
            syscalls::sys_write(1, &45, 1);
        }
        j = i - 1;
        while j >= 0 {
            c = arr[j as usize];
            syscalls::sys_write(1, &c, 1);
            j = j - 1;
        }
    }
}

impl OOutput1<u32> for Output
{
    fn o_put_1(&self, t: u32)
    {
        let mut num: u32 = t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        if num == 0 {
            arr[0] = 48;
            i = 1;
        }
        while num > 0 {
            arr[i as usize] = (num % 10) as u8 + 48;
            num = num / 10;
            i = i + 1;
        }
        j = i - 1;
        while j >= 0 {
            c = arr[j as usize];
            syscalls::sys_write(1, &c, 1);
            j = j - 1;
        }
    }
}

impl OOutput1<i32> for Output
{
    fn o_put_1(&self, t: i32)
    {
        let mut num: i32 = t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        let mut neg: bool = false;
        if num == 0 {
            arr[0] = 48;
            i = 1;
        }
        if num < 0 {
            neg = true;
            num = -num;
        }
        while num > 0 {
            arr[i as usize] = (num % 10) as u8 + 48;
            num = num / 10;
            i = i + 1;
        }
        if neg {
            syscalls::sys_write(1, &45, 1);
        }
        j = i - 1;
        while j >= 0 {
            c = arr[j as usize];
            syscalls::sys_write(1, &c, 1);
            j = j - 1;
        }
    }
}

impl OOutput1<u64> for Output
{
    fn o_put_1(&self, t: u64)
    {
        let mut num: u64 = t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        if num == 0 {
            arr[0] = 48;
            i = 1;
        }
        while num > 0 {
            arr[i as usize] = (num % 10) as u8 + 48;
            num = num / 10;
            i = i + 1;
        }
        j = i - 1;
        while j >= 0 {
            c = arr[j as usize];
            syscalls::sys_write(1, &c, 1);
            j = j - 1;
        }
    }
}

impl OOutput1<&i64> for Output
{
    fn o_put_1(&self, t: &i64)
    {
        let mut num: i64 = *t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        let mut neg: bool = false;
        if num == 0 {
            arr[0] = 48;
            i = 1;
        }
        if num < 0 {
            neg = true;
            num = -num;
        }
        while num > 0 {
            arr[i as usize] = (num % 10) as u8 + 48;
            num = num / 10;
            i = i + 1;
        }
        if neg {
            syscalls::sys_write(1, &45, 1);
        }
        j = i - 1;
        while j >= 0 {
            c = arr[j as usize];
            syscalls::sys_write(1, &c, 1);
            j = j - 1;
        }
    }
}

impl OOutput1<i64> for Output
{
    fn o_put_1(&self, t: i64)
    {
        let mut num: i64 = t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        let mut neg: bool = false;
        if num == 0 {
            arr[0] = 48;
            i = 1;
        }
        if num < 0 {
            neg = true;
            num = -num;
        }
        while num > 0 {
            arr[i as usize] = (num % 10) as u8 + 48;
            num = num / 10;
            i = i + 1;
        }
        if neg {
            syscalls::sys_write(1, &45, 1);
        }
        j = i - 1;
        while j >= 0 {
            c = arr[j as usize];
            syscalls::sys_write(1, &c, 1);
            j = j - 1;
        }
    }
}

impl OOutput1<f32> for Output
{
    fn o_put_1(&self, t: f32)
    {
        let mut num: f32 = t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        let mut neg: bool = false;
        if num == 0.0 {
            arr[0] = 48;
            i = 1;
        }
        if num < 0.0 {
            neg = true;
            num = -num;
        }
        while num > 0.0 {
            arr[i as usize] = (num % 10.0) as u8 + 48;
            num = num / 10.0;
            i = i + 1;
        }
        if neg {
            syscalls::sys_write(1, &45, 1);
        }
        j = i - 1;
        while j >= 0 {
            c = arr[j as usize];
            syscalls::sys_write(1, &c, 1);
            j = j - 1;
        }
    }
}

impl OOutput1<f64> for Output
{
    fn o_put_1(&self, t: f64)
    {
        let mut num: f64 = t;
        let mut arr: [u8; 20] = [0; 20];
        let mut i: i64 = 0;
        let mut j: i64;
        let mut c: u8;
        let mut neg: bool = false;
        if num == 0.0 {
            arr[0] = 48;
            i = 1;
        }
        if num < 0.0 {
            neg = true;
            num = -num;
        }
        while num > 0.0 {
            arr[i as usize] = (num % 10.0) as u8 + 48;
            num = num / 10.0;
            i = i + 1;
        }
        if neg {
            syscalls::sys_write(1, &45, 1);
        }
        j = i - 1;
        while j >= 0 {
            c = arr[j as usize];
            syscalls::sys_write(1, &c, 1);
            j = j - 1;
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
    ($($arg:expr),*) => {
        $(  
            Output.o_put_1($arg);
        )*
    };
}


fn main() {

    let mut a: i64 = 0;
    output!("Enter a number: ");
    input!(&mut a);

    let mut strin = String::new();
    output!("Enter a string: ");
    input!(&mut strin, 100, '\n');

    output!("The number is: ", a ,"\n");

    output!("The string is: ", strin, "\n");

    output!("The number is: ", a ,"\n");
}
