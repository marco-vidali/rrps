use rand::RngExt;
use std::{fmt, io};

struct Game {
    user_score: u8,
    computer_score: u8,
}

impl Game {
    fn new() -> Game {
        Game {
            user_score: 0,
            computer_score: 0,
        }
    }

    fn play_round(&mut self) {
        let user_move = get_user_move();

        clear_screen();

        let computer_move = calculate_computer_move();
        println!("The computer chose {computer_move}");

        determine_winner(self, &user_move, &computer_move);

        press_enter_to_continue();
    }
    fn display_score(&self) {
        println!("User: {}", self.user_score);
        println!("Computer: {}", self.computer_score);
    }
}

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
    clear_screen();

    println!("Welcome to RRPS (Rust Rock-Paper-Scissors)!");
    press_enter_to_continue();

    let mut game = Game::new();

    loop {
        game.play_round();
        game.display_score();
    }
}

fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}

fn press_enter_to_continue() {
    println!("Press enter to continue");

    let mut enter: String = String::new();

    io::stdin()
        .read_line(&mut enter)
        .expect("Failed to read user input");

    clear_screen();
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
                clear_screen();
                continue;
            }
        };

        break match user_move {
            1 => Move::Rock,
            2 => Move::Paper,
            3 => Move::Scissors,
            _ => {
                clear_screen();
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

fn determine_winner(game: &mut Game, user_move: &Move, computer_move: &Move) {
    match user_move {
        Move::Rock => match computer_move {
            Move::Rock => println!("Tie!"),
            Move::Paper => user_lost(game),
            Move::Scissors => user_won(game),
        },
        Move::Paper => match computer_move {
            Move::Rock => user_won(game),
            Move::Paper => println!("Tie!"),
            Move::Scissors => user_lost(game),
        },
        Move::Scissors => match computer_move {
            Move::Rock => user_lost(game),
            Move::Paper => user_won(game),
            Move::Scissors => println!("Tie!"),
        },
    };
}

fn user_won(game: &mut Game) {
    game.user_score += 1;
    game.computer_score -= 1;
    println!("You won!");
}

fn user_lost(game: &mut Game) {
    game.user_score -= 1;
    game.computer_score += 1;
    println!("You lost!");
}
