use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Type our guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
