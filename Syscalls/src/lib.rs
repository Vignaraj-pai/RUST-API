use std::arch::asm;
use libc;

#[repr(u64)]
pub enum LinuxSysCalls {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    WriteV = 20,
    Fork = 57,
    Execve = 59,
    Wait4 = 61,
    Clone = 56,
    Exit = 60,
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

pub fn sys_execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> i64 {
    unsafe { syscall(LinuxSysCalls::Execve as u64, path as u64, argv as u64, envp as u64) }
}

pub fn sys_wait4(pid: u64, status: *mut u64, options: u32, rusage: *mut u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Wait4 as u64, pid, status as u64, options as u64) }
}

pub fn sys_clone(flags: u64, stack: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Clone as u64, flags, stack, 0) }
}

pub fn sys_exit(status: u64) -> i64 {
    unsafe { syscall(LinuxSysCalls::Exit as u64, status, 0, 0) }
}

//USEFUL FUNCTIONS FOR THREADS

pub unsafe fn stack_create() -> u64 {
    let mut stack: u64;
    let stack_size: u64 = 4096 * 1024;
    asm!{
        "xor r9, r9",
        "xor rdi, rdi",
        "mov rax, 9",
        "syscall",
        "mov rdx, 0",
        "mov rsi, 1024",
        "mov rdi, rax",
        "mov rax, 10",
        "syscall",
        "add rdi, 4096 * 1024 + 1024 - 16",
        in("rsi") stack_size + 1024,
        in("rdx") libc::PROT_READ | libc::PROT_WRITE,
        in("r10") libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
        in("r8") -1,
        lateout("rdi") stack,
    };
    stack
}