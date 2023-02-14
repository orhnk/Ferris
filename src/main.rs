/* File: main.rs
 * Purpuse: Chess Game Logic code using self library
 * Author: KoBruhh
 * Date: 06.02.2023
 * */

mod board;
mod commandline;
mod parser;

use board::Board;
use board::Move;
use commandline::*;
use parser::convert_to_coords;
use std::{
    io::{stdin, Write},
    process::exit,
};

fn main() {
    let mut board: Board = Board::from_fen(
        "r   k  r                                                R   K  R".to_owned(),
    );
    board.draw(false);


    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut raw_coords = String::new();

        stdin()
            .read_line(&mut raw_coords)
            .expect("failed to readline");
        raw_coords = raw_coords.trim().to_string();

        match raw_coords.as_str() {
            "exit" => exit(0),
            "undo" => {
                board.undo_move();
                board.draw(false);
                continue;
            }
            "seval" => {
                println!("Evaluation: {}", board.simple_evaluate());
                continue;
            }
            "eval" => {
                println!("Evaluation: {}", board.evaluate());
                continue;
            }
            "reset" => {
                board = Default::default();
                board.draw(false);
                continue;
            }
            "clear" => {
                clear();
                board.draw(false);
                continue;
            }
            "turn" => {
                match board.turn {
                    true => println!("White's turn"),
                    false => println!("Black's turn"),
                };
                println!("Turn: {}", board.turn);
                continue;
            }
            _ => (),
        }

        if let Ok(coords) = convert_to_coords(&raw_coords) {
            let current_move = Move::new(coords[0], coords[1]); // move is a reserved keyword
            let moved = board.move_piece(current_move);
            if let Ok(_) = moved {
                // so annoying
                clear();
                board.draw(false);
            }
            if let Err(e) = moved {
                println!("{}", e);
            }
        }
    }
}
