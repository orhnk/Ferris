use crate::clear;
use crate::Board;
use std::io::{stdin, Write};
use std::process::exit;

pub fn command(board: &mut Board, raw_coords: String) -> Result<String, ()> {
    let turn = board.turn;
    let player = match turn {
        true => "White",
        false => "Black",
    };
    match raw_coords.as_str() {
        "exit" => exit(0),
        "undo" => {
            board.undo_move();
            return Err(());
        }
        "seval" => {
            println!("Evaluation: {}", board.simple_evaluate());
            return Err(());
        }
        "eval" => {
            println!("Evaluation: {}", board.evaluate());
            return Err(());
        }
        "reset" => {
            *board = board.classic();
            clear();
            return Err(());
        }
        "resign" => {
            println!("{player} resigned");
            exit(0);
        }
        "clear" => {
            clear();
            return Err(());
        }
        "turn" => {
            println!("Turn: {}", player);
            return Err(());
        }
        "pass" => {
            println!("Turn passed");
            board.reverse_turn();
            Err(())
        }
        "draw" => {
            println!("{player} offered a draw");
            println!("Accept? (y/n)");
            std::io::stdout().flush().unwrap();
            let mut accept = String::new();
            stdin().read_line(&mut accept).expect("failed to readline");
            accept = accept.trim().to_string();
            match accept.as_str() {
                "y" => {
                    println!("Draw accepted");
                    exit(0);
                }
                "n" => {
                    println!("Draw declined");
                    return Err(());
                }
                _ => {
                    println!("Invalid input");
                    return Err(());
                }
            }
        }
        _ => Ok(raw_coords),
    }
}
