/* File: board.rs
 * Author: KoBruhh
 * Purpuse: Constructing a chess board virtually
 * Date: 05.02.2023
 * */

mod chess_move;
mod color;
mod piece;

//use crate::board::color::{BoardColor, Color};
pub use chess_move::{Move, MoveErr, MoveType};
use color::*;
use piece::*;

const PIECE_SET: piece::Theme = piece::themes::CHALLENGER; // Or you can type (u8, u8, u8), (u8, u8, u8) instead
const BOARD_THEME: BTheme = color::themes::RUST;

static mut LAST_MOVE: ([usize; 2], [usize; 2]) = ([1, 2], [1, 2]);
static mut ESCAPE: &str = "\x1b[0m";

#[allow(non_upper_case_globals)]
const colored: bool = true; // will map it to command line argument thats why it is not SCREAMIN'
const PROMOTED: [char; 4] = ['q', 'r', 'b', 'n'];
const DEFAULT_PIECE_NOTATION: &str =
    "rnbqkbnrpppppppp                                PPPPPPPPRNBQKBNR"; // I have chosen to use something called FEN to encode FEN into board. This is fixed sized.
const BOUNDS: [usize; 2] = [1, 8]; // Bounds of the board
                                   /* FEN starts from left upper corner of the board and then all the way down to right bottom.
                                    * Lover-case characters (rnbqkp) represent black while upper-cases (RNBQKB) represent white FEN*/

#[allow(dead_code)] // can be used later
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

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub color: BoardColor,         // Used for storing the color of the board
    pub board: [[char; 8]; 8],     // Used for storing the board TODO
    pub FEN: String,               // Used for storing the FEN -> 72 is the max length of FEN
    pub turn: bool,                // White or black turn
    pub coordinates: bool,         // Used for displaying the coordinates
    pub white_color: piece::Color, // Used for storing the color of the white pieces
    pub black_color: piece::Color, // Used for storing the color of the black pieces
}

impl Board {
    #[allow(dead_code)]
    pub fn new(fen: String) -> Board {
        // Empty board
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

    #[allow(dead_code)]
    pub fn from_fen(fen: String) -> Board {
        let mut tmp = Board {
            color: BOARD_THEME.into(),
            board: [[' '; 8]; 8],
            FEN: fen,
            turn: true,
            coordinates: true,
            white_color: Default::default(),
            black_color: Default::default(),
        };
        tmp.decode();
        tmp
    }
    #[allow(dead_code)]
    pub fn from_vec(board: [[char; 8]; 8]) -> Board {
        let mut tmp = Board {
            color: BOARD_THEME.into(),
            board,
            FEN: String::new(),
            turn: true,
            coordinates: true,
            white_color: Default::default(),
            black_color: Default::default(),
        };
        tmp.encode();
        tmp
    }

    #[allow(dead_code)]
    pub fn resign(&mut self) {
        self.classic();
    }

    #[allow(dead_code)]
    pub fn classic(&mut self) -> Board {
        Board::default()
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, pos: [usize; 2]) {
        self.board[pos[0] - 1][pos[1] - 1] = ' ';
        self.encode();
    }

    #[allow(dead_code)]
    pub fn put(&mut self, pos: [usize; 2], piece: char) {
        self.board[pos[0] - 1][pos[1] - 1] = piece;
        self.encode();
    }

    #[allow(dead_code)]
    pub fn set_color(&mut self, color: BoardColor) {
        self.color = color;
    }

    #[allow(dead_code)]
    pub fn reverse_turn(&mut self) {
        self.turn = !self.turn;
    }

    #[allow(dead_code)]
    pub fn move_unchecked(&mut self, current_move: &Move) {
        // This is just for castling so We
        // don't have to clear the square

        let current_move = current_move.decode_move();
        let from = dbg!(current_move.0);
        let to = dbg!(current_move.1);

        dbg!(self.board[to[1] - 1][to[0] - 1]);
        dbg!(self.board[from[1] - 1][from[0] - 1]);

        self.board[to[1] - 1][to[0] - 1] = self.board[from[1] - 1][from[0] - 1];
        self.board[from[1] - 1][from[0] - 1] = ' ';
    }

    // TODO undo does not replace the taken piece
    pub fn move_piece(&mut self, current_move: Move) -> Result<MoveType, MoveErr> {
        let result = current_move.validate_move(&self.FEN, self.turn);

        if let Err(MoveErr(err)) = result {
            // for now I am ignoring
            return Err(MoveErr(err)); // TODO improve these to MoveErr
        }

        let result = result.unwrap();

        let (from, to) = current_move.decode_move();
        if from == to {
            return Err(MoveErr(
                "You can't move a piece to the same place".to_owned(),
            ));
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
        //        TODO Clean this shit. It's a mess
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
            return Err(MoveErr("Wait for your turn".to_owned())); // TODO! Doesn't work as it supposed to do
        }

        /* Castling involves 2 moves at a time,
         * This is against my programmes architechture,
         * This is why below function is ated in this file
         * * * * * * * * * * * * * * * * * * * * * * * * */

        if result == MoveType::Castle {
            /* Make sure that squares are empty! */
            // This will move rook
            //self.reverse_turn();
            /* Go to the corner which King has been moved */
            //self.encode();
            match self.turn {
                true => {
                    // White
                    // TODO Make this for the last ranks: 1, 8
                    if dbg!(current_move.decode_move().1[0]) == 7 {
                        // if we are trying to castle to the right
                        self.move_unchecked(&Move::new([8, 8], [6, 8]));
                    } else if dbg!(current_move.decode_move().1[0]) == 3 {
                        self.move_unchecked(&Move::new([1, 8], [4, 8]));
                    }
                }

                false => {
                    // Black
                    if dbg!(current_move.decode_move().1[0]) == 7 {
                        // if we are trying to castle to the right
                        self.move_unchecked(&Move::new([8, 1], [6, 1]));
                    } else if dbg!(current_move.decode_move().1[0]) == 3 {
                        self.move_unchecked(&Move::new([1, 1], [4, 1]));
                    }
                }
            };

            self.encode();
            self.move_unchecked(&current_move);
            self.decode();
        }

        let factor: i8 = match self.turn {
            true => -1,
            false => 1,
        };

        if result == MoveType::EnPassant {
            /*
             * This function is the seconds pass for en passand move type validation,
             * This will clearify if the taken pawn has been moved one move ago or not.
             * and this function will clear the take piece becuase en passand does not
             * replace the piece it takes.
             * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

            let taken_pawn = current_move.decode_move().1;
            let taken_pawn = [
                taken_pawn[0],
                (taken_pawn[1] as i8 + (1 * -factor)) as usize,
            ];

            // Verifying if the taken pawn has been moved one move ago
            unsafe {
                if LAST_MOVE.1 != taken_pawn {
                    return Err(MoveErr("Invalid en passant move".to_owned()));
                }
            }
            // Clearing the taken pawn
            self.remove(taken_pawn);
        }

        unsafe {
            LAST_MOVE = current_move.decode_move(); // Saving the last move
        }

        self.board[to[1] - 1_usize][to[0] - 1_usize] =
            self.board[from[1] - 1_usize][from[0] - 1_usize]; // Taking place of the piece
        self.turn = !self.turn; // Changing the turn
        self.board[from[1] - 1_usize][from[0] - 1_usize] = ' '; // Leaving moved piece's place empty
        return Ok(result);
    }

    pub fn undo_move(&mut self) {
        unsafe {
            self.board[LAST_MOVE.0[1] - 1_usize][LAST_MOVE.0[0] - 1_usize] =
                self.board[LAST_MOVE.1[1] - 1_usize][LAST_MOVE.1[0] - 1_usize];
            self.turn = !self.turn;
            self.board[LAST_MOVE.1[1] - 1_usize][LAST_MOVE.1[0] - 1_usize] = ' ';
        }
        self.encode();
    }

    pub fn draw_ascii(&mut self) {
        self.encode(); // Encode the board into FEN -> Because it's easier to display
                       // TODO:
                       // 1. Make this function flexible to draw any board
                       // 2. Add some piece sets to choose from
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

    pub fn promote_piece(&mut self, piece: char) -> Result<(), ()> {
        /*
         * Because this function has to get called after the piece moved,
         * We have to take !turn to get the moved piece's color
         * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
        let turn = !self.turn;
        let coords: [usize; 2];
        unsafe {
            coords = LAST_MOVE.1;
        }
        let piece = piece.to_ascii_lowercase();
        for i in PROMOTED {
            if i == piece {
                match turn {
                    true => self.board[coords[0] - 1][coords[1] - 1] = piece.to_ascii_uppercase(),
                    _ => self.board[coords[0] - 1][coords[1] - 1] = piece,
                }
                return Ok(());
            }
        }
        Err(())
    }

    #[allow(dead_code)]
    pub fn change_piece(&mut self, coords: [usize; 2], piece: char) {
        self.board[coords[0]][coords[1]] = piece;
    }

    //pub fn get_coordinate(&self, x: usize, y: usize) -> White {
    //White::Pawn
    //}
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

    #[test]
    fn test_board_from_fen() {
        let mut board = Board::default();
        board.encode();
        let mut board2 = Board::from_fen(board.FEN.clone());
        board2.encode();
        assert_eq!(board, board2);
    }

    #[test]
    fn test_board_from_vec() {
        let mut board = Board::default();
        board.encode();
        let mut board2 = Board::from_vec(board.board.clone());
        board2.encode();
        assert_eq!(board, board2);
    }
}
