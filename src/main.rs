use std::io;
use rand::Rng;

fn main() {

    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is: {}", secret_number);
    println!("Input your guess");

    let mut guess_number = String::new();

    io::stdin().read_line(&mut guess_number).expect("Failed to get guess number!");

    println!("Your guess number is: {}", guess_number);
}
