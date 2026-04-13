use std::io;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    clearscreen::clear().expect("Failed to clear screen");
    println!("Welcome to RRPS (Rust Rock-Paper-Scissors)!");

    loop {
        // Get a valid move from the user
        let user_move = loop {
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
        };
    }
}
