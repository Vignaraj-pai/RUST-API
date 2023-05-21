use std::env;

pub fn print_arguments() {
    let args: Vec<String> = env::args().collect();

    println!("Number of arguments: {}", args.len());

    for (index, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", index, arg);
    }
}

fn main() {
    print_arguments();
}