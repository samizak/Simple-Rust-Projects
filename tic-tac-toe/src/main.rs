use rand::Rng;
use std::{array, io};

const BOARD_SIZE: usize = 3;
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn main() {
    let available_characters = ['x', 'o'];

    println!("Welcome to tic-tac-toe!");
    println!(
        "Please choose between {} or {}: ",
        available_characters[0], available_characters[1]
    );

    let mut possible_choices: Vec<u32> = (1..=9).collect();
    let (user_char, computer_char) = get_character_choices(available_characters);
    let mut board: Board = generate_board();

    loop {
        print_board(&board);

        let user_choice = get_user_choice(&mut possible_choices);
        update_board(user_char, user_choice, &mut board);

        if check_win(user_char, &board) {
            print_board(&board);
            println!("Congratulations! You won!");
            break;
        }

        let ai_number: u32 = get_computer_choice(&mut possible_choices);
        update_board(computer_char, ai_number, &mut board);

        if check_win(computer_char, &board) {
            print_board(&board);
            println!("Computer wins!");
            break;
        }
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

fn generate_board() -> Board {
    array::from_fn(|row| {
        array::from_fn(|col| {
            let num = row * 3 + col + 1;
            char::from_digit(num as u32, 10).unwrap()
        })
    })
}

fn get_user_choice(possible_choices: &mut Vec<u32>) -> u32 {
    loop {
        println!("Available positions: {:?}", possible_choices);
        println!("Choose a position (1-9): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().parse() {
            Ok(num) if possible_choices.contains(&num) => {
                if let Some(pos) = possible_choices.iter().position(|&x| x == num) {
                    possible_choices.swap_remove(pos);
                    return num;
                }
            }
            Ok(_) => println!("Position already taken or invalid!"),
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn get_computer_choice(possible_choices: &mut Vec<u32>) -> u32 {
    let index = rand::thread_rng().gen_range(0..possible_choices.len());
    return possible_choices.swap_remove(index);
}

fn update_board(character: char, position: u32, board: &mut Board) {
    let index = position - 1;
    let row = (index / 3) as usize;
    let col = (index % 3) as usize;
    board[row][col] = character;
}

fn print_board(board: &Board) {
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

fn check_win(player: char, board: &Board) -> bool {
    // Check rows and columns
    for i in 0..BOARD_SIZE {
        if board[i].iter().all(|&c| c == player) || (0..BOARD_SIZE).all(|j| board[j][i] == player) {
            return true;
        }
    }

    // Check diagonals
    let diag1 = (0..BOARD_SIZE).all(|i| board[i][i] == player);
    let diag2 = (0..BOARD_SIZE).all(|i| board[i][BOARD_SIZE - 1 - i] == player);

    diag1 || diag2
}
