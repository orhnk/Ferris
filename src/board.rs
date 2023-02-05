/* File: board.rs
 * Author: KoBruhh
 * Purpuse: Constructing a chess board virtually
 * Date: 05.02.2023
 * */

mod piece;
use piece::Piece::*;

const DEFAULT_PIECE_NOTATION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"; // I have chosed to use something called FEN to encode pieces into board

/* FEN starts from left upper corner of the board and then all the way down to right bottom.
 * Lover-case characters (rnbqkp) represent black while upper-cases (RNBQKB) represent white pieces*/

pub struct Board {
    color: [(u8, u8, u8); 2],
    board: [[u8; 8]; 8],
    pieces: String,
    turn: bool,
    coordinates: bool,
}

impl Board {
    pub fn new(color: [(u8, u8, u8); 2], coord: bool) -> Board {
        Board {
            color,
            board: [[0; 8]; 8],
            pieces: String::from(DEFAULT_PIECE_NOTATION),
            turn: true,
            coordinates: coord,
        }
    }
    pub fn set_color(&mut self, color: [(u8, u8, u8); 2]) {
        self.color = color;
    }
    pub fn draw(&self) { // TODO:
                         // 1. Make this function flexible to draw any board
                         // 2. Add some piece sets to choose from
        const FRAME_VER:&str = "   |";
        const FRAME_HOR:&str = "-";
        const RTLB:&str = "|"; // Right Top Left Bottom
        const MARGIN:&str = " ";
        let size = 33;
        // Drawing board to the screen
        /* This is just experimental. I will convert this method to use OpenGL */
        let fen = &self.pieces;
        print!("{RTLB}");
        for piece in fen.chars() {
            match piece {
                'r' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Rook), //♜
                'n' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Knight), //♞
                'b' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Bishop), //♝
                'q' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Queen), //♛
                'k' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::King), //♚
                'p' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Pawn), //♟
                'R' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Rook),  //♖
                'N' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Knight), //♘
                'B' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Bishop), //♗
                'Q' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Queen), //♕
                'K' => print!("{MARGIN}{piece}{MARGIN}", piece = White::King),  //♔
                'P' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Pawn), //♙
                '/' => print!("\n{}\n", FRAME_HOR.repeat(size)),
                number => print!(
                    "{} {MARGIN} ",
                    FRAME_VER.repeat((number.to_digit(10).expect("Invalid FEN") - 1) as usize)
                ),
            }
            print!("{RTLB}");
        }
        println!("\n\nFEN: {}", fen);
    }
    fn encode(&self) {
        // Will convert board to FEN
        todo!()
    }
    fn decode(&self) {
        // Will convert FEN to board
        todo!()
    }
}
impl Default for Board {
    fn default() -> Self {
        Board {
            color: [(0, 0, 0), (255, 255, 255)],
            board: [[0; 8]; 8],
            pieces: String::from(DEFAULT_PIECE_NOTATION),
            turn: true, // white starts the game
            coordinates: true,
        }
    }
}
