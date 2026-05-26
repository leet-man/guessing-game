use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number! (1–100)");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut guesses: u32 = 0;

    loop {
        println!("Please input guess number {}", guesses + 1);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if !(1..=100).contains(&guess) {
            println!("Please guess a number between 1 and 100.");
            continue;
        }

        println!("You guessed: {guess}");
        guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You took {} guesses.", guesses);
                break;
            }
        }
    }
}