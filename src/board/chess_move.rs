/*
 * File: chess_move.rs
 * Purpose: Contains the Move struct and it's methods
 * Author: KoBruhh
 * Date: 11.02.2023
 * */


use super::Board;

pub enum MoveType {
    Normal,
    Capture,
    EnPassant,
    Castling,
    Promotion,
}

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
const KING_MOVE: [i8; 8] = [1, 8, -1, -8, 7, 9, -7, -9];
const KNIGHT_MOVE: [i8; 8] = [5, 11, 15, 17, -5, -11, -15, -17];
const PAWN_MOVE: [i8; 1] = [1];
const NM_PAWN_MOVE: [i8; 1] = [2];
const PAWN_CAPTURE: [i8; 2] = [7, 9];

// Contigious moves
const BISHOP_MOVE: [i8; 4] = [7, 9, -7, -9];
const ROOK_MOVE: [i8; 4] = [1, 8, -1, -8];
const QUEEN_MOVE: [i8; 8] = [1, 7, 8, 9, -1, -7, -8, -9];

pub struct Move([usize; 2], [usize; 2]); // start(x, y), end(x, y)

impl Move {
    pub fn new(start: [usize; 2], end: [usize; 2]) -> Self {
        Move(start, end)
    }

    pub fn decode_move(&self) -> ([usize; 2], [usize; 2]) {
        (self.0, self.1)
    }

    pub fn validate_move(&self, fen: &String, turn: bool) -> bool {
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

        let offset = self.to_fen();

        true // TODO
    }

    pub fn to_fen(&self) -> i32 {
        // Expected outputs:
        // [3, 4], [4, 4] -> 1
        // [3, 4], [4, 5] -> 9
        // [2, 1], [1, 3] -> -6
        // [x1, y1], [x2, y2] -> (x1 + y1 * 8) - (x2 + y2 * 8)]
        (self.1[0] + self.1[1] * 8) as i32 - (self.0[0] + self.0[1] * 8) as i32
    }

    pub fn get_move_type(&self, board: [[char; 8]; 8]) -> MoveType {
        todo!();
    }

    pub fn rate_move(&self, board: [[char; 8]; 8]) -> i32 {
        // evaulate how good was the move based on the board
        todo!();
    }
}

impl From<((usize, usize), (usize, usize))> for Move {
    fn from(m: ((usize, usize), (usize, usize))) -> Self {
        Move([m.0 .0, m.0 .1], [m.1 .0, m.1 .1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let m = Move::new([3, 4], [4, 4]);
        assert_eq!(m.to_fen(), 1);
        let m = Move::new([3, 4], [4, 5]);
        assert_eq!(m.to_fen(), 9);
        let m = Move::new([2, 1], [1, 3]);
        assert_eq!(m.to_fen(), 15);
    }

    #[test]
    fn test_into() {
        let m: Move = ((3, 4), (4, 4)).into();
        assert_eq!(m.to_fen(), 1);
        let m: Move = ((3, 4), (4, 5)).into();
        assert_eq!(m.to_fen(), 9);
        let m: Move = ((2, 1), (1, 3)).into();
        assert_eq!(m.to_fen(), 15);
    }
}
