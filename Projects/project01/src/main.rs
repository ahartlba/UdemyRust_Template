use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number!");

        let guessed_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guessed_number);

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("{}", "To small".red()),
            Ordering::Equal => {
                println!("{}", "Found!".green());
                break;
            }
            Ordering::Greater => println!("{}", "To big".red()),
        }
    }
}
