/* File: board.rs
 * Author: KoBruhh
 * Purpuse: Constructing a chess board virtually
 * Date: 05.02.2023
 * */

mod chess_move;
mod color;
mod piece;

//use crate::board::color::{BoardColor, Color};
pub use chess_move::{
    Move,
    MoveErr,
    MoveType,
};
use piece::*;
use color::*;


const PIECE_SET: piece::Theme = piece::themes::HACKER; // Or you can type (u8, u8, u8), (u8, u8, u8) instead
const BOARD_THEME: BTheme = color::themes::GRUVBOX;

static mut ESCAPE: &str = "\x1b[0m";
static mut last_move: ([usize; 2], [usize; 2]) = ([1, 2], [1, 2]);

const colored: bool = true; // will map it to command line argument
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
    color: BoardColor,     // Used for storing the color of the board
    board: [[char; 8]; 8], // Used for storing the board TODO
    FEN: String,           // Used for storing the FEN -> 72 is the max length of FEN
    turn: bool,            // White or black turn
    coordinates: bool,     // Used for displaying the coordinates
    white_color: piece::Color,    // Used for storing the color of the white pieces
    black_color: piece::Color,    // Used for storing the color of the black pieces
}

impl Board {
    pub fn new(fen: String) -> Board {
        Board {
            color: BOARD_THEME.into(),
            board: [[' '; 8]; 8],
            FEN: fen,
            turn: true,
            coordinates: true,
            white_color: Default::default(),
            black_color: Default::default(),
        }
    }

    pub fn set_color(&mut self, color: BoardColor) {
        self.color = color;
    }

    // TODO undo does not replace the taken piece
    pub fn move_piece(&mut self, current_move: Move) -> Result<MoveType, MoveErr> {
        let result = current_move.validate_move(&self.FEN, self.turn);
        if let Err(MoveErr(err)) = result { // for now I am ignoring
            return Err(MoveErr(err)); // TODO improve these to MoveErr
        }

        let result = result.unwrap();

        let (from, to) = current_move.decode_move();
        if from == to {
            return Err(MoveErr("You can't move a piece to the same place".to_owned()));
        }
        //if from or to includes any 0 or 9, then it's invalid
        for i in 0..BOUNDS.len() {
            if from[i] < BOUNDS[0_usize]
                || from[i] > BOUNDS[1_usize]
                || to[i] < BOUNDS[0_usize]
                || to[i] > BOUNDS[1_usize]
            {
                return Err(MoveErr("Invalid coordinates".to_owned()));
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
            return Err(MoveErr("Wait your turn and don't touch empty squares".to_owned())); // TODO! Doesn't work as it supposed to do
        }

        unsafe {
            last_move = current_move.decode_move(); // Saving the last move
        }

        self.board[to[1] - 1_usize][to[0] - 1_usize] =
            self.board[from[1] - 1_usize][from[0] - 1_usize]; // Taking place of the piece
        self.turn = !self.turn; // Changing the turn
        self.board[from[1] - 1_usize][from[0] - 1_usize] = ' '; // Leaving moved piece's place empty
        return Ok(result);
    }

    pub fn undo_move(&mut self) {
        unsafe {
            self.board[last_move.0[1] - 1_usize][last_move.0[0] - 1_usize] =
                self.board[last_move.1[1] - 1_usize][last_move.1[0] - 1_usize];
            self.turn = !self.turn;
            self.board[last_move.1[1] - 1_usize][last_move.1[0] - 1_usize] = ' ';
        }
        self.encode();
    }

    pub fn draw(&mut self, gui: bool) {
        self.encode(); // Encode the board into FEN -> Because it's easier to display
                       // TODO:
                       // 1. Make this function flexible to draw any board
                       // 2. Add some piece sets to choose from
        if gui {
            todo!();
        } else {
            const FRAME_HOR: &str = "";
            const FRAME_VER: &str = ""; // Right Top Left Bottom
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
                        fg_color = Into::<color::Color>::into(self.black_color).foreground();
                    } else {
                        fg_color = Into::<color::Color>::into(self.white_color).foreground();
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
                    // Unsafe blocks are needed because of use of global variables
                    ' ' => unsafe { print!("{bg_color}{MARGIN}{fg_color} {MARGIN}{ESCAPE}") },
                    'r' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = Black::Rook
                        )
                    }, //♜
                    'n' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = Black::Knight
                        )
                    }, //♞
                    'b' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = Black::Bishop
                        )
                    }, //♝
                    'q' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = Black::Queen
                        )
                    }, //♛
                    'k' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = Black::King
                        )
                    }, //♚
                    'p' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = Black::Pawn
                        )
                    }, //♟
                    'R' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = White::Rook
                        )
                    }, //♖
                    'N' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = White::Knight
                        )
                    }, //♘
                    'B' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = White::Bishop
                        )
                    }, //♗
                    'Q' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = White::Queen
                        )
                    }, //♕
                    'K' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = White::King
                        )
                    }, //♔
                    'P' => unsafe {
                        print!(
                            "{bg_color}{MARGIN}{fg_color}{piece}{MARGIN}{ESCAPE}",
                            piece = White::Pawn
                        )
                    }, //♙
                    _ => panic!("Invalid FEN"),
                }
                print!("{FRAME_VER}");

                row += 1;
                if row % 8 == 0 {
                    print!("{}\n", FRAME_HOR.repeat(size));
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

    pub fn evaluate(&self) -> i32 {
        // Will evaluate the board (with a depth search)
        todo!();
    }

    pub fn get_coordinate(&self, x: usize, y: usize) -> White {
        White::Pawn
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut init = Board {
            color: BOARD_THEME.into(),
            board: [[' '; 8]; 8],
            FEN: String::from(DEFAULT_PIECE_NOTATION),
            turn: true, // white starts the game
            coordinates: true,
            white_color: PIECE_SET.0,
            black_color: PIECE_SET.1,
        };
        init.decode();
        init
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_FEN() {
        let mut board = Board::default();
        board.encode();
        assert_eq!(board.FEN, DEFAULT_PIECE_NOTATION);
    }
}
