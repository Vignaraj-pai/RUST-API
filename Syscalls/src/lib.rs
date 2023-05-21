use std::{arch::asm, net::SocketAddr};

#[repr(u64)]
pub enum LinuxSysCalls {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    WriteV = 20,
    Fork = 57,
}

pub unsafe fn syscall(num: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        lateout("rax") res,
    );
    res
}

pub fn sys_write(fd: u64, data: *const u8, len: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Write as u64, fd, data as u64, len, 0, 0, 0) }
}

pub fn sys_read(fd: u64, buf: *mut u8, size: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Read as u64, fd, buf as u64, size as u64, 0, 0, 0) }
}

pub fn sys_open(path: *const u8, flags: u32, umode: u16) -> i64 {
    unsafe {
        syscall(
            LinuxSysCalls::Open as u64,
            path as u64,
            flags as u64,
            umode as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_close(fd: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Close as u64, fd, 0, 0, 0, 0, 0) }
}

pub fn sys_fork() -> i64 {
    unsafe { syscall(LinuxSysCalls::Fork as u64, 0, 0, 0, 0, 0, 0) }
}

pub fn sys_socket(domain: u32, type_: u32, protocol: u32) -> i64 {
    unsafe { syscall(41, domain as u64, type_ as u64, protocol as u64, 0, 0, 0) }
}

pub fn sys_bind(sockfd: u64, addr: &std::net::SocketAddr, addrlen: u32) -> i64 {
    let raw_ptr_to_enum: *mut std::net::SocketAddr = Box::into_raw(addr.clone().into());
    unsafe { syscall(49, sockfd, raw_ptr_to_enum as u64, addrlen as u64, 0, 0, 0) }
}

pub fn sys_listen(sockfd: u64, backlog: u32) -> i64 {
    unsafe { syscall(50, sockfd, backlog as u64, 0, 0, 0, 0) }
}

pub fn sys_accept(sockfd: u64, addr: &SocketAddr, addrlen: &u32) -> i64 {
    let raw_ptr_to_enum: *mut SocketAddr = Box::into_raw(addr.clone().into());
    unsafe { syscall(43, sockfd, raw_ptr_to_enum as u64, *addrlen as u64, 0, 0, 0) }
}

pub fn sys_connect(sockfd: u64, addr: &SocketAddr, addrlen: u32) -> i64 {
    let raw_ptr_to_enum: *mut SocketAddr = Box::into_raw(addr.clone().into());
    unsafe { syscall(42, sockfd, raw_ptr_to_enum as u64, addrlen as u64, 0, 0, 0) }
}

pub fn sys_sendto(
    sockfd: u64,
    buf: *const u8,
    len: u64,
    flags: u32,
    dest_addr: *const u8,
    addrlen: u32,
) -> i64 {
    unsafe {
        syscall(
            44,
            sockfd,
            buf as u64,
            len,
            flags as u64,
            dest_addr as u64,
            addrlen as u64,
        )
    }
}

pub fn sys_recvfrom(
    sockfd: u64,
    buf: *mut u8,
    len: u64,
    flags: u32,
    src_addr: *mut u8,
    addrlen: *mut u32,
) -> i64 {
    unsafe {
        syscall(
            45,
            sockfd,
            buf as u64,
            len,
            flags as u64,
            src_addr as u64,
            addrlen as u64,
        )
    }
}

pub fn sys_exit(status: u64) -> i64 {
    unsafe { syscall(60, status, 0, 0, 0, 0, 0) }
}
