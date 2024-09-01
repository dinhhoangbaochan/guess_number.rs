use std::io;

fn main() {

    println!("Welcome to the Guessing Game!");
    println!("Input your guess");

    let mut guess_number = String::new();

    io::stdin().read_line(&mut guess_number).expect("Failed to get guess number!");

    println!("Your guess number is: {}", guess_number);
}
