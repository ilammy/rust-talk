extern crate hello_world_2;

use std::io;

use hello_world_2::fib;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("read_line() error");

        match input.trim().parse() {
            Ok(n) => {
                println!("fib({}) = {}", n, fib(n));
            }
            Err(_) => {
                eprintln!("not an integer: {}", input);
                break;
            }
        }
    }
}

