use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret_number is {secret_number}");
    println!("Please, input a number from 0-100");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    let guess: u32 = guess.trim().parse().expect("Please use an integer!");
    println!("You guessed: {guess}");
}
