extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new(); 

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number was: {}", secret_number);
    println!("You guessed: {}", guess);

}
