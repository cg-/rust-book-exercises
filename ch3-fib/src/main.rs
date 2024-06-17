// Generate the nth Fibonacci number.
use std::io;

fn main() {
    println!("Input a value:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

    let n: u64 = n.trim().parse().expect("Couldn't convert.");

    for i in 0..n{
        print!("{},",fib(i));
    }
    print!("{}",fib(n));
}

fn fib(val: u64) -> u64{
    match val{
        0 => {
            return 0;
        },
        1 => {
            return 1;
        },
        _ => {
            return fib(val-1) + fib(val-2);
        },
    }
}
