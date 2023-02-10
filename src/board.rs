/* File: board.rs
 * Author: KoBruhh
 * Purpuse: Constructing a chess board virtually
 * Date: 05.02.2023
 * */

mod piece;
mod chess_move;
mod color;

use crate::board::color::{
    BoardColor,
    Color,
};
use piece::Piece::*;
pub use chess_move::Move;

static mut ESCAPE:&str = "\x1b[0m";

const DEFAULT_WHITE_PIECE_COLOR: (u8, u8, u8) = (20, 55, 255);
const DEFAULT_BLACK_PIECE_COLOR: (u8, u8, u8) = (0, 0, 0);

const colored:bool = true; // will map it to command line argument
const DEFAULT_PIECE_NOTATION: &str =
"rnbqkbnrpppppppp                                PPPPPPPPRNBQKBNR"; // I have chosen to use something called FEN to encode FEN into board. This is fixed sized.
const BOUNDS: [usize; 2] = [1, 8]; // Bounds of the board
/* FEN starts from left upper corner of the board and then all the way down to right bottom.
 * Lover-case characters (rnbqkp) represent black while upper-cases (RNBQKB) represent white FEN*/

fn nums_to_whitespaces(lit: &String) -> String {
    let mut s = String::with_capacity(lit.len());
    lit.chars().for_each(|ch| {
        if let Ok(num) = ch.to_string().parse::<u8>() {
            s.push_str(&" ".repeat(num as usize));
        } else {
            s.push(ch);
        }
    });
    s
}

pub struct Board {
    color: BoardColor,        // Used for storing the color of the board
    board: [[char; 8]; 8],    // Used for storing the board TODO
    FEN: String,              // Used for storing the FEN -> 72 is the max length of FEN
    turn: bool,               // White or black turn
    coordinates: bool,        // Used for displaying the coordinates
    white_color: Color,       // Used for storing the color of the white pieces
    black_color: Color,       // Used for storing the color of the black pieces
}

impl Board {

    pub fn new(color: BoardColor, coord: bool) -> Board {
        Board {
            color,
            board: [[' '; 8]; 8],
            FEN: String::from(DEFAULT_PIECE_NOTATION),
            turn: true,
            coordinates: coord,
            white_color: Color::new(255, 255, 255),
            black_color: Color::new(0, 0, 0),
        }
    }

    pub fn set_color(&mut self, color: BoardColor) {
        self.color = color;
    }

    pub fn move_piece(&mut self, current_move: Move) {
        current_move.validate_move();
        let (from, to) = current_move.decode_move();
        if from == to {
            panic!("You can't move a piece to the same place");
        }
        //if from or to includes any 0 or 9, then it's invalid
        for i in 0..BOUNDS.len() {
            if from[i] < BOUNDS[0_usize] || from[i] > BOUNDS[1_usize] || to[i] < BOUNDS[0_usize] || to[i] > BOUNDS[1_usize] {
                panic!("Invalid coordinates");
            }
        }
        // TODO
        //        self.board[to.0 as usize][to.1 as usize] = self.board[from.0 as usize][from.1 as usize];
        //        self.board[from.0 as usize][from.1 as usize] = 0;
        if self.board[from[1] - 1_usize][from[0] - 1_usize] == ' '
            || (self.board[from[1] - 1_usize][from[0] - 1_usize]
                .to_lowercase()
                .collect::<Vec<char>>()[0]
                != self.board[from[1] - 1_usize][from[0] - 1_usize]
                && self.turn == false)
                || (self.board[from[1] - 1_usize][from[0] - 1_usize]
                    .to_lowercase()
                    .collect::<Vec<char>>()[0]
                    == self.board[from[1] - 1_usize][from[0] - 1_usize]
                    && self.turn == true)
        {
            // If the piece is empty or if the piece is black and it's white's turn or if the piece is white and it's black's turn
            return (); // TODO! Doesn't work as it supposed to do
        }
        self.board[to[1] - 1_usize][to[0] - 1_usize] =
            self.board[from[1] - 1_usize][from[0] - 1_usize]; // Taking place of the piece
        self.turn = !self.turn; // Changing the turn
        self.board[from[1] - 1_usize][from[0] - 1_usize] = ' '; // Leaving moved piece's place empty
    }

    pub fn draw(&mut self, gui: bool) {
        self.encode(); // Encode the board into FEN -> Because it's easier to display
                       // TODO:
                       // 1. Make this function flexible to draw any board
                       // 2. Add some piece sets to choose from
        if gui {
            todo!();
        } else {
            const FRAME_HOR: &str = "-";
            const FRAME_VER: &str = "|"; // Right Top Left Bottom
            const MARGIN: &str = " ";
            let size = 40;
            // Drawing board to the screen
            /* This is just experimental. I will convert this method to use OpenGL */
            let fen = &self.FEN;
            let mut bg_color; 
            let mut fg_color; 

            let mut row = 0;
            let mut column = 0;
            println!("{}", FRAME_HOR.repeat(size));
            for piece in fen.chars() {
                print!("{FRAME_VER}");
                if colored {
                    if piece.is_lowercase() {
                        fg_color = self.white_color.foreground();
                    } else {
                        fg_color = self.black_color.foreground();
                    }
                    if (row + column / 8) % 2 == 0 {
                        bg_color = self.color.rgb().0.background();
                    } else {
                        bg_color = self.color.rgb().1.background();
                    }
                } else {
                    unsafe {
                        ESCAPE = "";
                    }
                    bg_color = "".to_owned();
                    fg_color = "".to_owned();
                }
                column += 1; // Incrementing column

                match piece {
                    ' ' => unsafe { print!("{bg_color}{MARGIN}{fg_color} {MARGIN}{ESCAPE}") },
                    'r' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = Black::Rook) }, //♜
                    'n' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = Black::Knight) }, //♞
                    'b' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = Black::Bishop) }, //♝
                    'q' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = Black::Queen) }, //♛
                    'k' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = Black::King) }, //♚
                    'p' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = Black::Pawn) }, //♟
                    'R' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = White::Rook) }, //♖
                    'N' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = White::Knight) }, //♘
                    'B' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = White::Bishop) }, //♗
                    'Q' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = White::Queen) }, //♕
                    'K' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = White::King) }, //♔
                    'P' => unsafe { print!("{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}", piece = White::Pawn) }, //♙
                    _ => panic!("Invalid FEN"),
                }
                print!("{FRAME_VER}");

                row += 1;
                if row % 8 == 0 {
                    print!("\n{}\n", FRAME_HOR.repeat(size));
                    row = 0;
                }
            }
            println!("\n\nFEN: {}", fen);
        }
    }

    pub fn encode(&mut self) {
        // Will convert board to FEN
        let mut tmp = String::with_capacity(72);
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                tmp.push(self.board[i][j]);
            }
        }
        self.FEN = tmp;
    }

    pub fn decode(&mut self) {
        // Will convert FEN to board
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.board[i][j] = self.FEN.chars().nth(i * 8 + j).expect("Invalid FEN");
            }
        }
    }

    pub fn simple_evaluate(&self) -> i32 {
        // Will evaluate the board (without a depth search)
        let mut score = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    'r' => score -= 5,
                    'n' => score -= 3,
                    'b' => score -= 3,
                    'q' => score -= 9,
                    'p' => score -= 1,
                    'R' => score += 5,
                    'N' => score += 3,
                    'B' => score += 3,
                    'Q' => score += 9,
                    'P' => score += 1,
                    _ => (),
                }
            }
        }
        score
    }

    pub fn evalue(&self) -> i32 {
        // Will evaluate the board (with a depth search)
        todo!();
    }

}

impl Default for Board {
    fn default() -> Self {
        let mut init = Board {
            color: Default::default(),
            board: [[' '; 8]; 8],
            FEN: String::from(DEFAULT_PIECE_NOTATION),
            turn: true, // white starts the game
            coordinates: true,
            white_color: Color::new(DEFAULT_WHITE_PIECE_COLOR.0, DEFAULT_WHITE_PIECE_COLOR.1, DEFAULT_WHITE_PIECE_COLOR.2),
            black_color: Color::new(DEFAULT_BLACK_PIECE_COLOR.0, DEFAULT_BLACK_PIECE_COLOR.1, DEFAULT_BLACK_PIECE_COLOR.2),
        };
        init.decode();
        init
    }
}
