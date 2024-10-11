use rand::Rng;
use std::{cmp::Ordering, io}; // package for reading input & ordering // random number generator
fn main() {
    println!("Guess the number:");

    loop {
        println!("Please input your guess.");

        // by default, variables are immutable. meaning once a value is set
        // it can't be changed. If we set it mutable,
        // it means we can change the values.
        let mut guess = String::new();

        // reading the input into the string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // before comparing, convert guess to integer
        // we're shadowing, meaning we're modifying the original guess variable
        // instead of creating a new one.
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        // generate a secret random number
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is {secret_number}");

        // compare the input and the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
