use rand::Rng;
use std::io;

fn main() {
    let mut number_of_tries = 3;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    while number_of_tries > 0 {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().parse::<u32>().is_err() {
            println!("Enter a valid number!");
            continue;
        }

        let guess: u32 = guess.trim().parse().expect("Not a number");

        if guess != secret_number {
            println!("Wrong! Try again...");
            number_of_tries = number_of_tries - 1;
        } else {
            println!("Correct! Well done!");
            return;
        }
    }
    println!("Game Over, you lost!");
}
