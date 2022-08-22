use std::cmp::Ordering;
use std::io;

// Rng is a trait
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // rand is a library crate that provides a random number generator.
    // import in Cargo.toml
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        /*
        1. let: declares a new variable binding
        2. mut: declare that variables are mutable, default is immutable
        3. new() is a function belong String type (Correlation function, means like static function)
        */
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).
            // read_line will return io::Result, contains OK and Err, we need use expect to deal with Err
            expect("Failed to read line");

        // : u32 means specified uint32
        // parse -> guess string type try to specified type u32
        // parse will return Result type, contains OK and Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} is a placeholder
        println!("You guessed: {}", guess);

        // match had three arm, secret_number will try match pattern (Less, Greater, Equal)
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
