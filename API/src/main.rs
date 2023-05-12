
fn main() {
    let string = "Hello, world!\n";
    syscalls::sys_fork();
    syscalls::sys_write(1, string.as_ptr(), string.len() as u64);
}
