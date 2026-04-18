use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number!");

    println!("Please input your guess");
    let rng = &mut rand::rng();
    let secret_number = rng.random_range(1..100);
    println!("The secret number is: {}", secret_number);
    loop {
        let mut gue
        ss = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to take the input");
        println!("You guessed: {}", guess);
        let parsed_guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
        println!(
            "good {}, the secret number is: {}",
            parsed_guess, secret_number
        );

        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
