use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
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
    let mut inp: i64 = 0;
    let mut line: String = "".to_string();
    input!(&mut inp);
    input!(&mut line, 100, '\n');
    let mut s = String::new();
    s.push_str(&inp.to_string());
    
    output!("The number you entered is: " s "\n" "The sentence you entered is: " line "\n");
    
}
