use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("secret_number is {secret_number}");
    loop {
        println!("Please, input a number from 0-100");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) if num <= 100 => num,
            Ok(_) => {
                println!("Please enter a number between 0 and 100.");
                continue;
            }
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            cmp::Ordering::Greater => println!("The number I am thinking is shorter!"),
            cmp::Ordering::Less => println!("The number I am thinking is greater!"),
            cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
