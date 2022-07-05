// Chapter 2 - Programming a Guessing Game

use std::io;

pub fn guessing_game() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to rad line!");

    println!("You guessed: {guess}");
}
