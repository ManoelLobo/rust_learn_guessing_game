use std::io;

fn main() {
    println!("Guess the number");

    println!("Type our guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read input");

    println!("Your guess is {guess}")
}
