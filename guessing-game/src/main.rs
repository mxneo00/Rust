use std::io; // input/output library from the standard library
use rand::Rng; // random number generator from the rand crate
use std::cmp::Ordering; // ordering enum for comparing values
fn main() {
    println!("Guess the number!");

    // Generate a random secret number from 1 to 100 (inclusive).
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    // Keep asking for guesses until the player gets it right.
    loop {
        println!("Please input your guess.");

        // Store the user's raw input text.
        let mut guess: String = String::new();
        io::stdin() 
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim spaces/newlines and parse the input as a number.
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", guess);

        // Compare the guess with the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
