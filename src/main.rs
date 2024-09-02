use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is: {}", secret_number);
    println!("Input your guess");

    let mut guess_number = String::new();

    io::stdin().read_line(&mut guess_number).expect("Failed to get guess number!");

    // this one is called `variable shadowing`
    // it lets us reuse the previous variable
    // guess_number was a string, need to convert to i32, an int with 32-bit, same as secret_number, so we can compare.
    let guess_number: u32 = guess_number.trim().parse().unwrap();

    println!("Your guess number is: {}", guess_number);
    
    // infinite loop
    loop {

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Wrong! Seems like you should try a bigger number!"),
            Ordering::Equal => println!("Congratulations! You guess the correct number!"),
            Ordering::Greater => println!("Wrong! Maybe try a smaller number?")
        }

    }
}
