use std::io;
use rand::Rng;

fn main() {
    // first comment
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess:");

    // create mutable variable `guess`
    // type is String and associated with function
    // new.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed the number: {}", guess);
}
