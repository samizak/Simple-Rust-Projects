use rand::Rng;
use std::{array, io};

const BOARD_SIZE: usize = 3;

fn main() {
    let available_characters = ['x', 'o'];

    println!("Welcome to tic-tac-toe!");
    println!(
        "Please choose between {} or {}: ",
        available_characters[0], available_characters[1]
    );

    let mut possible_choices: Vec<u32> = (1..=9).collect();
    // Get user and computer character choices
    let (user_char, computer_char) = get_character_choices(available_characters);
    // Generate board options
    let mut board: [[char; BOARD_SIZE]; BOARD_SIZE] = generate_board();

    loop {
        print_board(&mut board);

        // Get the user number choice
        let user_number = get_user_number_choice(&mut possible_choices);
        println!("You chose: {}", user_number);

        // Update board with player's
        update_board(user_char, user_number, &mut board);

        let ai_number: u32 = get_computer_choice(&mut possible_choices);
        println!("Computer chose: {ai_number}");

        // Update board with computer's
        update_board(computer_char, ai_number, &mut board);
    }
}

fn get_character_choices(available_characters: [char; 2]) -> (char, char) {
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

fn generate_board() -> [[char; BOARD_SIZE]; BOARD_SIZE] {
    array::from_fn(|row| {
        array::from_fn(|col| {
            let num = row * 3 + col + 1;
            char::from_digit(num as u32, 10).unwrap()
        })
    })
}

fn get_user_number_choice(possible_choices: &mut Vec<u32>) -> u32 {
    loop {
        println!("Choose a number between 1-9");
        let mut user_choice = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read");

        match user_choice.trim().parse::<u32>() {
            Ok(num) => {
                if possible_choices.contains(&num) {
                    possible_choices.retain(|&x| x != num);
                    return num;
                } else {
                    println!("Enter a valid number between 1 and 9 that wasn't already used!");
                }
            }
            Err(_) => println!("'{user_choice}' is not a valid number!"),
        }
    }
}

fn update_board(char_to_use: char, board_index: u32, board: &mut [[char; BOARD_SIZE]; BOARD_SIZE]) {
    let index = board_index - 1;
    let row = (index / 3) as usize;
    let col = (index % 3) as usize;
    board[row][col] = char_to_use;
}

fn get_computer_choice(possible_choices: &mut Vec<u32>) -> u32 {
    let index = rand::thread_rng().gen_range(1..possible_choices.len());
    return possible_choices.swap_remove(index);
}

fn print_board(board: &mut [[char; BOARD_SIZE]; BOARD_SIZE]) {
    println!("\nCurrent Board:");

    for (i, row) in board.iter().enumerate() {
        let row_str = row
            .iter()
            .map(|c| format!(" {:^3} ", c))
            .collect::<Vec<_>>()
            .join("|");

        println!("{}", row_str);

        if i < BOARD_SIZE - 1 {
            println!("{}", ["-----"; BOARD_SIZE].join("+"));
        }
    }
}
