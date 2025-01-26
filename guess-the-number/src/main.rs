use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    guessing_game_v1();
    guessing_game_v2();
}

fn guessing_game_v1() {
    let mut number_of_tries = 3;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number (1-100)! You have {number_of_tries} tries:");

    while number_of_tries > 0 {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        };

        if guess < secret_number {
            println!("Too low!");
        } else if guess > secret_number {
            println!("Too high!");
        } else {
            println!("Correct! Well done!");
            return;
        }

        number_of_tries -= 1;
        println!("{} tries remaining", number_of_tries);
    }
    println!("Game Over, you lost!");
}

fn guessing_game_v2() {
    let number_of_tries: i32 = 3;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number (1-100)!");

    for remaining in (1..=number_of_tries).rev() {
        let mut input = String::new();
        println!("{remaining} tries left:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let Ok(guess) = input.trim().parse::<u32>() else {
            println!("Invalid number!");
            continue;
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }

    println!("Game over! Number was {secret_number}")
}
