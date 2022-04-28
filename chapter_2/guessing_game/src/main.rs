use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // create an immutable variable
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        // create a mutable variable
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number.");
                continue;
            },
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win. Exact match!");
                break;
            },
            Ordering::Less => println!("To small. Try a bigger number."),
            Ordering::Greater => println!("To big. Try a smaller number."),
        }
    }
}
