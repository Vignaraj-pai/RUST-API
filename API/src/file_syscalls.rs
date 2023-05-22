
enum FileFlags {
    O_RDONLY = 0,
    O_WRONLY = 1,
    O_RDWR = 2,
    O_CREAT = 64,
    O_APPEND = 1024,
}

enum FileModes {
    S_IRUSR = 256,
    S_IWUSR = 128,
    S_IXUSR = 64,
    S_IRGRP = 32,
    S_IWGRP = 16,
    S_IXGRP = 8,
    S_IROTH = 4,
    S_IWOTH = 2,
    S_IXOTH = 1,
}

trait OpenFile<T,U>
{
    fn open_file(t: T,u: U)->i64;
}

trait OpenFileWithMode<T, U, V>
{
    fn open_file_with_mode(t: T, u: U, v: V)->i64;
}

struct OpenFileSyscall;

impl OpenFile<String,FileFlags> for OpenFileSyscall
{
    fn open_file(t: String, u: FileFlags) -> i64 {
        let fd:i64 = syscalls::sys_open(t.as_ptr(),u as u32,FileModes::S_IRUSR as u16);
        return fd;
    }
}

impl OpenFileWithMode<String,FileFlags,FileModes> for OpenFileSyscall
{
    fn open_file_with_mode(t: String, u: FileFlags, m: FileModes) -> i64 {
        let fd:i64 = syscalls::sys_open(t.as_ptr(), u as u32, m as u16);
        return fd;
    }
}

macro_rules! open_file {
    ($arg1:expr,$arg2:expr) => {
        OpenFileSyscall::open_file($arg1,$arg2);
    };

    ($arg1:expr, $arg2:expr, $arg3:expr) => {
        OpenFileSyscall::open_file_with_mode($arg1,$arg2,$arg3);
    }
}

fn main() {
    let string:String = String::from("/workspaces/RUST-API/API/src/text.txt");
    let fd = open_file!(string, FileFlags::O_CREAT);
    if(fd<0)
    {
        println!("Error in finding or opening file having error code: {}",fd);
    }
    println!("File Descriptor{}",fd);
}
