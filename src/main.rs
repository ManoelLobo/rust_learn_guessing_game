use std::{cmp::Ordering, io};

use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("The secret number should be between 1 and 100, you chose {value}");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    println!("Guess the number (from 1 to 100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Type our guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("You guessed lower"),
            Ordering::Greater => println!("You guessed higher"),
            Ordering::Equal => {
                println!("You nailed it");
                break;
            }
        }
    }
}
