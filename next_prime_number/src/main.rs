extern crate rand;

use std::io;
use rand::Rng;

fn check_prime(num: i32) -> bool {
    let num1 = &num;
    let mut t_or_f: bool = true;
    for i in 2..*num1 {
        if num1 % i == 0 {
            t_or_f = false;
            break
        }
    }
    t_or_f
}

fn all_primes() {
    println!("Please input the maximum number for prime numbers, e.g. 100");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("There was an error");
    match num.trim().parse::<i32>() {
        Ok(_g) => {
            let num = num.trim().parse::<i32>().unwrap();
            println!("1st prime number = 2");
            for i in (3..num+1).step_by(2) {
                if check_prime(i) == true {
                    println!("Next prime number = {}", i);
                }
            } 
        }

        Err(err) => println!("There was an error {}", err)
    } 
}


fn main() {
    println!("Hello, world!");
    all_primes();
}
