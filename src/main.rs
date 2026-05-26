use std::{
    cmp::Ordering, // Provides Less, Greater, Equal for comparisons
    io::{self, Write}, // `io` for stdin/stdout, `Write` trait for flush()
};

use rand::Rng; // Provides gen_range() for random number generation

fn main() {
    println!("Guess the number! (1–100)");

    // Generate a random integer between 1 and 100 (inclusive)
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut guesses: u32 = 0;

    loop {
        // print! (unlike println!) doesn't append a newline, keeping
        // the cursor on the same line for input
        print!("Guess #{}: ", guesses + 1);

        // Flush stdout so the prompt appears before read_line() blocks
        io::stdout().flush().unwrap();

        let mut input = String::new();

        // Block until the user presses Enter, appending the result to `input`
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Attempt to parse the trimmed input as u32;
        // silently skip the iteration if it fails
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Reject guesses outside the valid range without counting them
        if !(1..=100).contains(&guess) {
            println!("Please guess a number between 1 and 100.");
            continue;
        }

        guesses += 1;

        // Compare guess to secret_number and respond accordingly
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} — too small!"),
            Ordering::Greater => println!("{guess} — too big!"),
            Ordering::Equal => {
                println!("You win! You took {} guesses.", guesses);
                break; // Exit the loop on a correct guess
            }
        }
    }
}
