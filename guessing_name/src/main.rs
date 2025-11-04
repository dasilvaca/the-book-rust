use rand::Rng;
use std::cmp;
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

    match guess.cmp(&secret_number) {
        cmp::Ordering::Greater => println!("The number I am thinking is shorter!"),
        cmp::Ordering::Less => println!("The number I am thinking is greater!"),
        cmp::Ordering::Equal => println!("You guessed right!"),
    }
    if guess < secret_number {
        println!("number is greater!");
    } else if guess > secret_number {
        println!("number is shorter!");
    } else {
        println!("you guessed right!");
    }
}
