use rand::Rng;
use std::io;

fn main() {
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3]; // 3x3 grid
    let available_characters = ['x', 'o'];

    println!("Welcome to tic-tac-toe!");
    println!(
        "Please choose between {} or {}: ",
        available_characters[0], available_characters[1]
    );

    let mut possible_choices: Vec<u32> = (1..=9).collect();

    // Get user and computer character choices
    let (user_char, computer_char) = get_user_and_computer_character_choices(available_characters);

    println!("\n");

    // Generate board options
    let mut counter: u32 = 1;
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            *cell = char::from_digit(counter, 10).expect("Failed to create board");
            counter += 1;
        }
    }

    loop {
        for row in board.iter() {
            let display_row = row
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" | ");
            println!(" {display_row}\n-----------");
        }

        // println!("{:?}", board);
        // println!("You go first!");

        let mut user_number = 0;

        loop {
            println!("Choose a number between 1-9");
            let mut user_choice = String::new();

            io::stdin()
                .read_line(&mut user_choice)
                .expect("Failed to read");

            match user_choice.trim().parse::<u32>() {
                Ok(num) => {
                    if possible_choices.contains(&num) {
                        user_number = num;
                        possible_choices.retain(|&x| x != num);
                        break;
                    } else {
                        println!("Enter a valid number between 1 and 9!");
                    }
                }
                Err(_) => {
                    println!("'{user_choice}' is not a valid number!");
                }
            }
        }

        println!("You chose: {}", user_number);

        let index = user_number - 1;
        let row = (index / 3) as usize;
        let col = (index % 3) as usize;
        board[row][col] = user_char;

        let mut ai_number = rand::thread_rng().gen_range(1..=9);

        loop {
            if possible_choices.contains(&ai_number) {
                break;
            }
            ai_number = rand::thread_rng().gen_range(1..=9);
            possible_choices.retain(|&x| x != ai_number);
        }

        println!("Computer chose: {ai_number}");

        let index = ai_number - 1;
        let row = (index / 3) as usize;
        let col = (index % 3) as usize;
        board[row][col] = computer_char;

        println!("{:?}", possible_choices);
    }
}

fn get_user_and_computer_character_choices(available_characters: [char; 2]) -> (char, char) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let user_char: char = match input.len() {
            1 => input.chars().next().unwrap(),
            _ => {
                println!("Please enter exactly one character!");
                continue;
            }
        };

        if available_characters.contains(&user_char) {
            let computer_char = available_characters
                .iter()
                .find(|&&c| c != user_char)
                .unwrap();

            println!("You chose '{user_char}'");
            return (user_char, *computer_char);
        }

        println!(
            "'{}' is not valid. Choose '{}' or '{}'!",
            user_char, available_characters[0], available_characters[1]
        );
    }
}
