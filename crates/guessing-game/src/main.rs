use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..=10);

    loop {
        println!("Please, enter your guess:");

        let mut guess = String::new();

        if let Err(e) = io::stdin().read_line(&mut guess) {
            println!("Failed to read line {e:?}");
            continue;
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a whole number between 1 and 10.");
                continue;
            }
        };

        match secret_number.cmp(&guess) {
            Ordering::Less => println!("Too big!"),
            Ordering::Greater => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
