use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Secret number is: {}", secret_number);
    
    // infinite loop
    loop {
        let mut guess_number = String::new();

        println!("Input your guess");
        io::stdin().read_line(&mut guess_number).expect("Failed to get guess number!");

        let guess_number: u32 = guess_number.trim().parse().unwrap();

        println!("Your guess number is: {}", guess_number);

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Wrong! Seems like you should try a bigger number!"),
            Ordering::Greater => println!("Wrong! Maybe try a smaller number?"),
            Ordering::Equal => {
                println!("Congratulations! You guess the correct number!");
                break;
            }
        }

    }
}
