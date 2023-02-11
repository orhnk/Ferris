/*
 * File: chess_move.rs
 * Purpose: Contains the Move struct and it's methods
 * Author: KoBruhh
 * Date: 11.02.2023
 * */


// TODO: Undoing undone moves lead to a problem

//use crate::board::char_to_piece;
use std::error::Error;
use std::fmt::{Display, Formatter};

// I am planning to add custom dialog boxes for the game using this:

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MoveType {
    Regular,
    DoublePawn, // for the first move of a pawn
    PawnCapture, // Pawn's capture differently from other pieces
    Capture,
    EnPassant,
    Castling,
    Promotion,
}

#[derive(Clone)]
pub struct Move([usize; 2], [usize; 2]); // start(x, y), end(x, y)

#[derive(Debug, Clone)]
pub struct MoveErr(pub String);

/* An ascii Chess Board with pieces on:
 *   -----------------
 * 8| r n b q k b n r |
 * 7| p p p p p p p p |
 * 6|                 |
 * 5|                 |
 * 4|                 |
 * 3|                 |
 * 2| P P P P P P P P |
 * 1| R N B Q K B N R |
 *   -----------------
 *   a b c d e f g h
 *
 * I think I will use FEN notation to represent the board
 * that would be easier to parse and validate
 * Mine is a simpler version of:
 *      https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
 *
 *  *----------------------------------------------------------------*
 *
 * Default FEN notation for a chess board: (Simplified)
 *  "rnbqkbnrpppppppp                                PPPPPPPPRNBQKBNR";
 *
 *  -------------------------------------------------------------
 * |        |                                                    |
 * | Piece  |         offset according to FEN                    |
 * | King   | &(+1, -1), &(+7, -7), &(+8, -8), &(+9, -9)         |
 * | Queen  | (+1, -1), (+7, -7), (+8, -8), (+9, -9)             |
 * | Rook   | (+1, -1), (+8, -8)                                 |
 * | Bishop | (+7, -7), (+9, -9)                                 |
 * | Knight | &(+11, -11), &(+5, -5), &(+17, -17), &(+15, -15)   |
 * | Pawn   | &(+1), !&(+2), *&(+7), *&(+9)                      |
 * |        |                                                    |
 *  -------------------------------------------------------------
 * | * - capture          | >> Non-contigious moves, move will be
 * | ! - first move       |  considered as is, unlike other moves
 * | & - non-contigious   |  can move till board's edge
 *  ----------------------
 *
 * Now I am ignoring the castling and en passant moves. (sake of simplicity)
 * */

// TODO ensure that board in bounds

// THESE CHANGE DEPENDING ON THE COLOR OF THE PIECE ( WHITE(-) OR BLACK(+) )

// Non-contigious moves

/*   Independent on color */
const KING_MOVE: [i8; 8] = [1, 7, 8, 9, -1, -7, -8, -9];
const KNIGHT_MOVE: [i8; 8] = [5, 11, 15, 17, -5, -11, -15, -17];
    
/*   Dependent on color */
const PAWN_MOVE: [i8; 1] = [8];
const NM_PAWN_MOVE: [i8; 1] = [16];
const PAWN_CAPTURE: [i8; 2] = [7, 9];

// Contigious moves
const BISHOP_MOVE: [i8; 4] = [7, 9, -7, -9];
const ROOK_MOVE: [i8; 4] = [1, 8, -1, -8];
const QUEEN_MOVE: [i8; 8] = [1, 7, 8, 9, -1, -7, -8, -9];


impl Display for MoveErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MoveErr {}

impl Move {

    pub fn new(start: [usize; 2], end: [usize; 2]) -> Self {
        Move(start, end)
    }

    pub fn decode_move(&self) -> ([usize; 2], [usize; 2]) {
        (self.0, self.1)
    }

    pub fn validate_move(&self, fen: &String, turn: bool) -> Result<MoveType, MoveErr> {
        // self.0 is the start position
        // self.1 is the end position
        /*
         * Rules:
         * 1. The piece can't move to the same position
         * 2. The piece can't move to a position where it's own piece is present
         * 3. The piece can't move to a position where it's opponent's piece is present
         * (except fot capture move)
         * 4. The piece can't move where it has no attribute to move
         * 5. Some pieces will not be able to move in check
         * 6. Some pieces will behave differently after moved for the first time
         * 7. King can't castle if any sqaure is under attack
         * */

        let factor = match turn {
            true => -1,
            false => 1,
        };

        let from_move_index = fen_idx(self.0);
        let to_move_index = fen_idx(self.1);
        let moved_piece = fen.chars().nth(from_move_index as usize - 1).expect("OUT OF BOUNDS");

        println!("Move index: {}", from_move_index);
        println!("Moved piece: {}", moved_piece);
        let diff = (to_move_index - from_move_index) as i8 * factor;

        println!("diff_fen: {}", diff);
        match moved_piece.to_ascii_uppercase() {
            'P' => {
                if PAWN_MOVE.contains(&diff) {
                    return Ok(MoveType::Regular);
                }
                else if NM_PAWN_MOVE.contains(&diff) {
                    return Ok(MoveType::DoublePawn);
                }
                else if is_full(fen, to_move_index) && PAWN_CAPTURE.contains(&diff) {
                    return Ok(MoveType::PawnCapture);
                } else {
                    return Err(MoveErr("Invalid Pawn move".to_owned()));
                }
            }
            'R' => {
                if ROOK_MOVE.contains(&diff) {
                    if is_full(fen, to_move_index) {
                        return Ok(MoveType::Capture);
                    } else {
                        return Ok(MoveType::Regular);
                    }
                } else {
                    return Err(MoveErr("Invalid Rook move".to_owned()));
                }
            }
            'N' => {
                if KNIGHT_MOVE.contains(&diff) {
                    if is_full(fen, to_move_index) {
                        return Ok(MoveType::Capture);
                    } else {
                        return Ok(MoveType::Regular);
                    }
                } else {
                    return Err(MoveErr("Invalid Knight move".to_owned()));
                }
            }
            'B' => {
                if BISHOP_MOVE.contains(&diff) {
                    if is_full(fen, to_move_index) {
                        return Ok(MoveType::Capture);
                    } else {
                        return Ok(MoveType::Regular);
                    }
                } else {
                    return Err(MoveErr("Invalid Bishop move".to_owned()));
                }
            }
            'Q' => {
                if QUEEN_MOVE.contains(&diff) {
                    if is_full(fen, to_move_index) {
                        return Ok(MoveType::Capture);
                    } else {
                        return Ok(MoveType::Regular);
                    }
                } else {
                    return Err(MoveErr("Invalid Queen move".to_owned()));
                }
            }
            'K' => {
                if KING_MOVE.contains(&diff) {
                    if is_full(fen, to_move_index) {
                        return Ok(MoveType::Capture);
                    } else {
                        return Ok(MoveType::Regular);
                    }
                } else {
                    return Err(MoveErr("Invalid King move".to_owned()));
                }
            }
            _ => {
                return Err(MoveErr("No valid piece found".to_owned()));
            }
        }
        //char_to_piece(moved_piece);
    }


    #[allow(dead_code)]
    pub fn moved_piece(&self, fen: &String) -> char {
        // There is a -1 because the index starts from 0
        fen.chars().nth(fen_idx(self.0) as usize - 1).expect("OUT OF BOUNDS") 
    }

    #[allow(dead_code)]
    pub fn diff_fen(&self) -> i32 {
        // Expected outputs:
        // [3, 4], [4, 4] -> 1
        // [3, 4], [4, 5] -> 9
        // [2, 1], [1, 3] -> -6
        // [x1, y1], [x2, y2] -> (x1 + y1 * 8) - (x2 + y2 * 8)]
        fen_idx(self.1) - fen_idx(self.0)
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn rate_move(&self, fen: &String) -> i32 {
        // evaulate how good was the move based on the board
        todo!();
    }
}


fn fen_idx(moved: [usize; 2]) -> i32 {
    // [x, y] -> (x + (y - 1) * 8)
    (moved[0] + (moved[1] - 1) * 8) as i32 
}

fn is_full(fen:&String, to_move_index: i32) -> bool {
        fen.chars().nth(to_move_index as usize - 1).expect("OUT OF BOUNDS:PAWN_CAPTURE") != ' ' 
}

impl From<((usize, usize), (usize, usize))> for Move {
    fn from(m: ((usize, usize), (usize, usize))) -> Self {
        Move([m.0 .0, m.0 .1], [m.1 .0, m.1 .1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Board;

    #[test]
    fn test_move() {
        let m = Move::new([3, 4], [4, 4]);
        assert_eq!(m.diff_fen(), 1);
        let m = Move::new([3, 4], [4, 5]);
        assert_eq!(m.diff_fen(), 9);
        let m = Move::new([2, 1], [1, 3]);
        assert_eq!(m.diff_fen(), 15);
    }

    #[test]
    fn test_into() {
        let m: Move = ((3, 4), (4, 4)).into();
        assert_eq!(m.diff_fen(), 1);
        let m: Move = ((3, 4), (4, 5)).into();
        assert_eq!(m.diff_fen(), 9);
        let m: Move = ((2, 1), (1, 3)).into();
        assert_eq!(m.diff_fen(), 15);
    }

    #[test]
    fn test_fen_idx() {
        assert_eq!(fen_idx([3, 4]), 27);
        assert_eq!(fen_idx([4, 4]), 28);
        assert_eq!(fen_idx([2, 1]),  2);
        assert_eq!(fen_idx([1, 3]), 17);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_moved_piece() {
        let mut board: Board = Default::default();
        let m = Move::new([5, 7], [5, 6]);
        board.move_piece(m.clone()); // This is testing purpuses only.
                                    // After piece moves, we don't 
                                    // want to keep the old position
        assert_eq!(m.moved_piece(&board.FEN), 'P');
        board.draw(false);

        let m = Move::new([5, 2], [5, 4]);
        board.move_piece(m.clone());
        assert_eq!(m.moved_piece(&board.FEN), 'p');
        board.draw(false);

        let m = Move::new([2, 8], [3, 6]);
        board.move_piece(m.clone());
        assert_eq!(m.moved_piece(&board.FEN), 'N'); // has to be 57 but it is 58
        board.draw(false);

        let m = Move::new([2, 1], [3, 3]);
        board.move_piece(m.clone());
        assert_eq!(m.moved_piece(&board.FEN), 'n');
        board.draw(false);

        let m = Move::new([6, 8], [3, 5]);
        board.move_piece(m.clone());
        assert_eq!(m.moved_piece(&board.FEN), 'B');
        board.draw(false);

        let m = Move::new([6, 1], [2, 5]);
        board.move_piece(m.clone());
        assert_eq!(m.moved_piece(&board.FEN), 'b');
        board.draw(false);

        /* uncomment this to see the steps of validation: 
         * `cargo test` to see validation steps after uncommented*/
         //assert!(false == true); // This returns false so test will output stdout
    }

}
