use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // 1..=100 is INCLUSIVE RANGE, meaning we include both 1 and 100
    // and create an array of numbers from 1 to 100
    // 1..100 is EXCLUSIVE RANGE, 100 is not included
    // gen_range method takes range as argument (start..=end)
    let secret_number = rand::thread_rng().gen_range(1..=50);

    println!("secretn number was: {secret_number}");
}