use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // first comment
    println!("Guess the number!");

    // using `rand` crate generate random number between
    // 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // loop through inputs to allow multiple guesses
    loop {
        println!("Please input your guess:");

        // create mutable variable `guess`
        // type is String and associated with function
        // new.
        let mut guess = String::new();
        
        // io library reads from command line
        // and adds the input to the mutable string
        //`guess`.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed the number: {}", guess);

        // shadow guess and convert to int so we can compare
        // against `secret_number`. Also catch errors when input
        // is not an integer.
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Error guess: '{}' could not be parsed as a integer, please try again!", guess.trim());
                continue;
            },
        };
        
        // compare guess against secret number 
        // and print message based on the comparison.
        // if we match the secret number then exit the loop.
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Nailed it!");
                break;
            }
        }
    }
}
