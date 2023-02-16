/* File: main.rs
 * Purpuse: Chess Game Logic code using self library
 * Author: KoBruhh
 * Date: 06.02.2023
 * */

mod board;
mod commandline;
mod parser;
mod commands;

use board::Board;
use board::Move;
use board::MoveType;
use commandline::*;
use parser::convert_to_coords;
use commands::command;

use std::{
    io::{stdin, Write},
};

/*
 * Buggy FENS:
 *  "r bq rk ppppnppp  n     bB  p       P     PP N  PP   PPPRNBQ RK "
 *  27 25 -> Errenous move
 * */

static mut MOVE_TYPE: String = String::new();

fn main() {
    //let mut board: Board = Board::from_fen(
        //"        P                                              p        ".to_owned(),
    //);
    let mut board = Board::default();

    loop {
        board.draw_ascii();
        unsafe {
            println!("Move type: {}", MOVE_TYPE);
        }
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut raw_coords = String::new();

        stdin()
            .read_line(&mut raw_coords)
            .expect("failed to readline");
        raw_coords = raw_coords.trim().to_string();

        let raw_coords = match command(&mut board, raw_coords) {
            Ok(raw_coords) => {
                raw_coords
            }
            _ => {
                continue;
            }
        };

        if let Ok(coords) = convert_to_coords(&raw_coords) {
            let current_move = Move::new(coords[0], coords[1]); // move is a reserved keyword
            let move_t = board.move_piece(current_move);
            match move_t {
                Ok(move_t) => {
                    if move_t == MoveType::Promotion {
                        println!("Promote to (q/r/b/n): ");
                        std::io::stdout().flush().unwrap();
                        let mut promote = String::new();
                        stdin()
                            .read_line(&mut promote)
                            .expect("failed to readline");
                        let promote:char = promote.trim().chars().nth(0).unwrap();
                        match board.promote_piece(promote) {
                            Err(_) => {
                                println!("Enter a valid piece");
                                board.undo_move();
                                continue;
                            }
                            _ => {}
                        }
                    }
                    unsafe {
                        MOVE_TYPE = move_t.to_string();
                    }
                    clear();
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
