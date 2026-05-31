use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number:)");

    let the_secret_number = rand::random_range(1..100);

    println!("the_secret_number is ={the_secret_number}");

    let mut guesses: i8 = 0;

    loop {
        guesses = guesses + 1;
        println!("Please enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        println!("You Guess {guess}");

        let guess: i8 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        match guess.cmp(&the_secret_number) {
            Ordering::Less => println!("Your guess is too low↓"),
            Ordering::Greater => println!("Your guess is too high↑"),
            Ordering::Equal => {
                println!("You own the game in {guesses} guesses :)");
                break;
            }
        }
    }
}
