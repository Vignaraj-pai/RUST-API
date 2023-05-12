use std::arch::asm;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(u64)]
pub enum LinuxSysCalls {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    WriteV = 20,
    Fork = 57,
}

#[repr(u32)]
#[allow(non_camel_case_types, unused)]
pub enum LinuxFileFlags {
    /// Open for reading only.
    O_RDONLY = 0o0,
    /// Open for writing only.
    O_WRONLY = 0o1,
    /// Opens a file for reading and writing.
    O_RDWR = 0o2,
    /// Create file if it doesn't exist.
    O_CREAT = 0o100,
    /// Append if file has content.
    O_APPEND = 0o2000,
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

pub fn sys_writev(fd: u64, iovec: *const u8, vlen: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::WriteV as u64, fd, iovec as u64, vlen) }
}

pub fn writev<const N: usize>(fd: u64, msgs: &[&CStr]) -> i64 {
    #[derive(Copy, Clone)]
    #[repr(C)]
    struct iovec {
        iov_base: *const c_char,
        len: usize,
    }
    impl Default for iovec {
        fn default() -> Self {
            Self {
                iov_base: std::ptr::null(),
                len: 0,
            }
        }
    }

    let mut vector: [iovec; N] = [iovec::default(); N];
    // copy the C-string pointers into the iovec-array
    for (i, cstr) in msgs.iter().enumerate() {
        vector[i].iov_base = cstr.as_ptr();
        vector[i].len = cstr.to_bytes().len()
    }
    // execute the syscall
    sys_writev(fd, vector.as_ptr() as *const u8, msgs.len() as u64)
}

pub fn sys_close(fd: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Close as u64, fd, 0, 0) }
}

pub fn sys_fork() -> i64 {
    unsafe { syscall(LinuxSysCalls::Fork as u64, 0, 0, 0) }
}
