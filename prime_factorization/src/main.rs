extern crate rand;

use std::io;


fn _simple_prime_factorization(num: u32) {
    let num1 = &num;
    let mut prime: u32 = 2;
        while num1 >= &(prime*prime) {
            if num1%prime == 0 {
                print!("{} ", prime);
                let _num1 = &(num1/prime);
            } else {
                prime = prime+1;
            }
        }
        print!("{}", num1);
}


fn prime_factorization() {
    println!("What number do you want to prime factorize");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("There was an error");
    match num.trim().parse::<usize>() {
        Ok(_g) => {let mut num1 = num.trim().parse::<usize>().unwrap(); 
        let mut prime = 2;
        print!("Primes are: ");
        while num1 >= prime*prime {
            if num1%prime == 0 {
                print!("{} ", prime);
                num1 = num1/prime;
            } else {
                prime = prime+1;
            }
        }
        print!("{}", num1)
    }

        Err(err) => println!("There was an error! {}", err)

}
}

fn main() {
    println!("Hello, world!");
    prime_factorization();
}
