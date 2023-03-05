use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;
use std::mem::replace;

fn main() {
    loop {
        println!("Enter the a number to retrieve nth value in the fibbonaci sequence: (Enter \"quit\" to exit)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read input.");

        if input.trim() == "quit" {
            println!("Quitting...");
            break;
        }

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        //if input <= 93 {
        //    let vect = fibbonaci(input, vector);
        //    let input = input as usize;
        //    println!("{}", vect[input])
        //} else {
        //    println!("Value too high")
        //}
        let answer = fibbonaci(input);
        println!("{}", answer)
    }
}

fn fibbonaci(input: usize) -> BigUint {
    // let mut x_0: BigUint = BigUint::from(0u32);
    let mut x_0: BigUint = Zero::zero();

    // let mut x_1: BigUint = BigUint::from(1u32);
    let mut x_1: BigUint = One::one();

    for _ in 0..input {
        let x_2 = x_0 + &x_1;

        // pub fn replace<T>(dest: &mut T, src: T)
        // Moves src into the referenced dest, returning the previous dest value.
        x_0 = replace(&mut x_1, x_2);
    }
    return x_0;
}
