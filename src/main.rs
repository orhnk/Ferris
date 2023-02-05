/* File: main.rs
 * Purpuse: Chess Game Logic code using self library
 * Author: KoBruhh
 * Date: 06.02.2023
 * */

mod board;
mod parser;
use board::Board;
use parser::convert_to_coords;
use std::io::{stdin, Write};

fn main() {
        let mut board: Board = Default::default();
        board.draw(false);
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut raw_coords = String::new();

        stdin()
            .read_line(&mut raw_coords)
            .expect("failed to readline");
        if let Ok(coords) = convert_to_coords(&raw_coords){
        board.move_piece(coords[0], coords[1]);
        board.draw(false);
        }
    }
}
