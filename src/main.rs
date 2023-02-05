mod board;
use board::Board;

fn main() {
    let mut board:Board = Default::default();
    board.draw();
}
