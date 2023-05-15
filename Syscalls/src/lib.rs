use std::arch::asm;

#[repr(u64)]
pub enum LinuxSysCalls {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    WriteV = 20,
    Fork = 57,
}

pub unsafe fn syscall(num: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") res,
    );
    res
}

pub fn sys_write(fd: u64, data: *const u8, len: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Write as u64, fd, data as u64, len) }
}

pub fn sys_read(fd: u64, buf: *mut u8, size: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Read as u64, fd, buf as u64, size as u64) }
}

pub fn sys_open(path: *const u8, flags: u32, umode: u16) -> i64 {
    unsafe {
        syscall(
            LinuxSysCalls::Open as u64,
            path as u64,
            flags as u64,
            umode as u64,
        )
    }
}

pub fn sys_close(fd: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Close as u64, fd, 0, 0) }
}

pub fn sys_fork() -> i64 {
    unsafe { syscall(LinuxSysCalls::Fork as u64, 0, 0, 0) }
}
