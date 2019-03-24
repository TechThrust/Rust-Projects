extern crate rand;

use std::io;
use rand::Rng;

fn fibonacci_sequence() {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    println!("To what nth term do you want the Fibonnaci Sequence");
    let mut nth = String::new();
    io::stdin().read_line(&mut nth).expect("There was an error");
    let nth_num = nth.trim().parse::<usize>();
    match nth_num {
        Ok(_g) => {for i in 0..nth_num.unwrap() {
            if i % 2 == 0 {
                a = a+b;
                println!("{}th term is {}", i, a)
            }
            if i % 2 == 1 {
                b = b+a;
                println!("{}th term is {}", i, b)
            }
        }},

        Err(err) => {println!("There was an error, maybe you did not enter number {}", err)},
    }
}

fn main() {
    println!("Hello, world!");
    fibonacci_sequence();
}
