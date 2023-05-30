use rand::{self, Rng};
use std::{cmp::Ordering, io};

fn main() {
    println!("Welcome to Guess The Number Game");
    println!("-----------");

    let secret_number = rand::thread_rng().gen_range(0..101);
    let mut counter = 0;
    loop {
        let mut input_line = String::new();
        println!("Please enter your guess: ");
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read input");

        let input_number: i32 = match input_line.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match input_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!(
                    "Congratulations, you've found the number in {} guesses!",
                    counter
                );
                break;
            }
            Ordering::Less => println!("The secret number is GREATER"),
            Ordering::Greater => println!("The secret number is LESSER"),
        }
        counter += 1;
    }
}
