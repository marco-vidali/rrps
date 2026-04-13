use rand::RngExt;
use std::{fmt, io};

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Rock => write!(f, "rock"),
            Move::Paper => write!(f, "paper"),
            Move::Scissors => write!(f, "scissors"),
        }
    }
}

fn main() {
    clearscreen::clear().expect("Failed to clear screen");
    println!("Welcome to RRPS (Rust Rock-Paper-Scissors)!");

    loop {
        let user_move = get_user_move();

        let computer_move = calculate_computer_move();
        println!("The computer chose {computer_move}");

        determine_winner(&user_move, &computer_move);
    }
}

fn get_user_move() -> Move {
    loop {
        println!("Choose a move:");
        println!("1) Rock");
        println!("2) Paper");
        println!("3) Scissors");

        let mut user_move = String::new();

        io::stdin()
            .read_line(&mut user_move)
            .expect("Failed to read user input");

        let user_move: i8 = match user_move.trim().parse() {
            Ok(user_move) => user_move,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        break match user_move {
            1 => Move::Rock,
            2 => Move::Paper,
            3 => Move::Scissors,
            _ => {
                println!("Please enter a valid choice");
                continue;
            }
        };
    }
}

fn calculate_computer_move() -> Move {
    match rand::rng().random_range(1..=3) {
        1 => Move::Rock,
        2 => Move::Paper,
        3 => Move::Scissors,
        _ => unreachable!("random_range(1..=3) should only return 1, 2 or 3"),
    }
}

fn determine_winner(user_move: &Move, computer_move: &Move) {
    match user_move {
        Move::Rock => match computer_move {
            Move::Rock => println!("Tie!"),
            Move::Paper => println!("You lost!"),
            Move::Scissors => println!("You won!"),
        },
        Move::Paper => match computer_move {
            Move::Rock => println!("You won!"),
            Move::Paper => println!("Tie!"),
            Move::Scissors => println!("You lost!"),
        },
        Move::Scissors => match computer_move {
            Move::Rock => println!("You lost!"),
            Move::Paper => println!("You won!"),
            Move::Scissors => println!("Tie!"),
        },
    };
}
