use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number (from 1 to 100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Type our guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Same as
        // if guess < 1 || guess > 100
        if !(1..=100).contains(&guess) {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("Your guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed lower"),
            Ordering::Greater => println!("You guessed higher"),
            Ordering::Equal => {
                println!("You nailed it");
                break;
            }
        }
    }
}
