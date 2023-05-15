trait OInput<T>
{
    fn o_take_inp_1(&self, t: T);
}

struct Input;
impl Input
{
    fn take_inp_1<T>(&self, t: T) where Self: OInput<T>
    {
        self.o_take_inp_1(t);
    }
}

impl OInput<&mut u64> for Input
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

impl OInput<&mut i64> for Input
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

macro_rules! input {
    ($arg:expr) => {
        Input.take_inp_1($arg);
    };
}

fn main() {
    let mut inp: i64 = 0;
    input!(&mut inp);
    println!("{}", inp);
}
