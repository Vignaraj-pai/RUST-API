use libc;
use std::arch::asm;

//INPUT
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

impl OInput1<&mut char> for Input
{
    fn o_take_inp_1(&self, t: &mut char)
    {
        let mut c: u8 = 0;
        syscalls::sys_read(0, &mut c, 1);
        *t = c as char;
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
//end INPUT


//PROCESS MANAGEMENT

static mut PIDS: Vec<u64> = Vec::new();

//forks
fn fork() -> i64
{
    //create static variable to store the pids of the children
    unsafe
    {
        let pid: i64 = syscalls::sys_fork();
        if pid == 0 {
            //child
            return 0;
        }
        else if pid > 0 {
            //parent
            PIDS.push(pid as u64);
            return pid;
        }
        return pid;
    }
}

trait JoinAll
{
    fn join_all_i(&self);
}

trait JoinOne
{
    fn join_i(&self, pid: u64);
}

struct Join;
impl Join
{
    fn join_all(&self) where Self: JoinAll
    {
        self.join_all_i();
    }

    fn join(&self, pid: u64) where Self: JoinOne
    {
        self.join_i(pid);
    }
}

impl JoinAll for Join
{
    fn join_all_i(&self)
    {
        unsafe
        {
            let mut pid: u64;
            let mut status: u64 = 0;
            let mut i: usize = 0;
            let mut len: usize;
            loop {
                len = PIDS.len();
                if i >= len {
                    break;
                }
                pid = PIDS[i];
                let status_ptr: *mut u64 = &mut status;
                syscalls::sys_wait4(pid, status_ptr, 0, 0 as *mut u64);
                i = i + 1;
            }
        }
    }
}

impl JoinOne for Join
{
    fn join_i(&self, pid: u64)
    {
        let mut status: u64 = 0;
        let status_ptr: *mut u64 = &mut status;
        //check if pid is in PIDS
        // unsafe
        // {
        //     let mut i: usize = 0;
        //     let mut len: usize;
        //     loop {
        //         len = PIDS.len();
        //         if i >= len {
        //             break;
        //         }
        //         if PIDS[i] == pid {
        //             break;
        //         }
        //         i = i + 1;
        //     }
        // }
        syscalls::sys_wait4(pid, status_ptr, 0, 0 as *mut u64);
    }
}

macro_rules! join {
    () => {
        Join.join_all();
    };

    ($arg:expr) => {
        Join.join($arg);
    }
}

unsafe fn exit_thread()
{
    asm!{
        "pop rax",
        "sub rsp, 4096 * 1024 + 1024",
        "mov rsi, 4096 * 1024",
        "mov rdi, rsp",
        "mov rax, 11",
        "syscall",
        "mov rax, 60",
        "syscall",
    }
    return;
}

unsafe fn new_thread(ptr: fn() -> ()) -> u64
{

    let stack_ptr: u64 = syscalls::stack_create();
    let exit: unsafe fn() -> () = exit_thread;
    let flags: u64 = (libc::CLONE_VM | libc::CLONE_FS | libc::CLONE_FILES | libc::CLONE_SIGHAND | libc::CLONE_PARENT |libc::CLONE_THREAD | libc::CLONE_IO) as u64;
    asm!{
        "push r14",
        "push rax",
        "pop qword ptr [rdi + 8]",
        "pop qword ptr [rdi]",
        in("rdi") stack_ptr,
        in("rax") exit,
        in("r14") ptr,
    };
    let tid: u64;
    asm!{
        "xor r8, r8",
        "xor r10, r10",
        "xor rdx, rdx",
        "syscall",
        in("rdi") flags,
        in("rsi") stack_ptr,
        in("rax") 56,
        lateout("r10") tid,
    };
    return tid;
}

fn test_thread()
{
    print!("Hello from thread\n");
    return;
}

fn main() {
    let pid: u64;
    unsafe { pid = new_thread(test_thread); }
    print!("Hello from main\n");
    join!(pid);
}