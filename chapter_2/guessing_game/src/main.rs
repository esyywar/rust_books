use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() { 
    println!("Guess the number! (between 1-100)");
    let rand_num = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess:");

        // explicitly declare 'mut' for variables that change value (immutable by default)
        let mut guess = String::new();

        // would call this as std::io::stdin if not for the 'use' import at top
        let num_bytes_read = io::stdin()
                            .read_line(&mut guess)
                            .expect("failed to read line");

        println!("read {num_bytes_read} bytes of input");

        // rust shadowing feature allows re-using a variable name when type is different
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
              println!("invalid number");
              continue;
            },
        };

        println!("You guessed: {guess}");

        // match is like switch-case statement
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("too low"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
            Ordering::Greater => println!("too high"),
        };
    }

    println!("Random number was {rand_num}");
}
