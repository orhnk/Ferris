/*
 * File: chess_move.rs
 * Purpose: Contains the Move struct and it's methods
 * Author: KoBruhh
 * Date: 11.02.2023
 * */

// TURN SYSTEM HAS BUGS!

// TODO: Undoing undone moves lead to a problem
// TODO: Castling
// TODO: En passant
// TODO: Promotion
// TODO: Check and Checkmate
// TODO: Stalemate
// TODO: Resign
// TODO: Draw
// TODO: Moves doesn't affect the board if not drawed (BUG)

//use crate::board::char_to_piece;
use std::error::Error;
use std::fmt::{Display, Formatter};

// I am planning to add custom dialog boxes for the game using this:

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum MoveType {
    Regular,
    DoublePawn,  // for the first move of a pawn
    PawnCapture, // Pawn's capture differently from other pieces
    Capture,

    EnPassant, // TODO
    Castle,    // TODO
    Promotion, // TODO
}

#[derive(Clone)]
pub struct Move([usize; 2], [usize; 2]); // start(x, y), end(x, y)

#[derive(Debug, Clone)]
pub struct MoveErr(pub String);

#[allow(dead_code)]
enum Offset {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
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

// NEW
// Non-contigious moves
/*   Independent on color */
/*  */

// OLD
// Non-contigious moves
/*   Independent on color */
const KING_MOVE: [i8; 8] = [1, 7, 8, 9, -1, -7, -8, -9];
const KNIGHT_MOVE: [i8; 8] = [5, 11, 15, 17, -5, -11, -15, -17];

/*   Dependent on color */
const PAWN_MOVE: [i8; 1] = [8];
const NM_PAWN_MOVE: [i8; 1] = [16];
const PAWN_CAPTURE: [i8; 2] = [7, 9];
const CASTLING_KING: [i8; 2] = [2, -2];
const CASTLING_ROOK: [i8; 2] = [3, -4];

// Contigious moves
const BISHOP_MOVE: [i8; 4] = [7, 9, -7, -9];
const ROOK_MOVE: [i8; 4] = [1, 8, -1, -8];
const QUEEN_MOVE: [i8; 8] = [1, 7, 8, 9, -1, -7, -8, -9];

static mut TAKEN_PIECE: Option<char> = None;

impl Display for MoveErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MoveErr {}

impl Offset {
    pub fn to_i32(&self) -> i32 {
        match self {
            Offset::Up => 8,
            Offset::Down => -8,
            Offset::Left => -1,
            Offset::Right => 1,
            Offset::UpLeft => 7,
            Offset::UpRight => 9,
            Offset::DownLeft => -9,
            Offset::DownRight => -7,
        }
    }
}

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

        let from_move_index = fen_idx(self.0) as i8;
        let to_move_index = fen_idx(self.1);
        let taken_piece = fen
            .chars()
            .nth(to_move_index as usize - 1)
            .expect("OUT OF BOUNDS");

        unsafe {
            TAKEN_PIECE = match taken_piece {
                ' ' => None,
                _ => Some(taken_piece),
            };
        }
        let moved_piece = fen
            .chars()
            .nth(from_move_index as usize - 1)
            .expect("OUT OF BOUNDS");

        println!("Moving {}, to {}", from_move_index, to_move_index);
        println!("Moved piece: {}", moved_piece);
        let diff = (to_move_index - from_move_index as i32) as i8 * factor;

        println!("diff_fen: {}", diff);
        match moved_piece.to_ascii_uppercase() {
            'P' => {
                if non_contigious_move_check(PAWN_MOVE.to_vec(), diff)
                    && is_empty(fen, to_move_index)
                {
                    return Ok(MoveType::Regular);
                } else if non_contigious_move_check(NM_PAWN_MOVE.to_vec(), diff)
                    && is_empty_till_n(fen, to_move_index, Offset::Up, 2, turn)
                {
                    return Ok(MoveType::DoublePawn);
                } else if is_full(fen, to_move_index) && PAWN_CAPTURE.contains(&diff) {
                    return Ok(MoveType::PawnCapture);
                } else {
                    return Err(MoveErr("Invalid Pawn move".to_owned()));
                }
            }
            'R' => {
                if contigious_move_check(ROOK_MOVE.to_vec(), from_move_index, diff, fen, factor) {
                    /* I need some way to represent Contigious moves */
                    if is_full(fen, to_move_index) {
                        // TODO Does not check if there is a piece in between
                        return Ok(MoveType::Capture);
                    } else {
                        return Ok(MoveType::Regular);
                    }
                } else {
                    return Err(MoveErr("Invalid Rook move".to_owned()));
                }
            }
            'N' => {
                // Knight can move in L shape and can jump over other pieces
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
                // Bishop can move diagonally. TODO Does not check if there is a piece in between
                if contigious_move_check(BISHOP_MOVE.to_vec(), from_move_index, diff, fen, factor) {
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
                if contigious_move_check(QUEEN_MOVE.to_vec(), from_move_index, diff, fen, factor) {
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
                if dbg!(CASTLING_KING.contains(&diff)) {
                    // Will couse error. (left is empty it can castle right)
                    if (is_empty(fen, to_move_index)
                        && is_empty(fen, to_move_index + 1 * factor as i32))
                        || (is_empty(fen, -to_move_index)
                            && is_empty(fen, -to_move_index + 1 * factor as i32))
                    {
                        // TODO Add here
                        return Ok(MoveType::Castle);
                    } else {
                        return Err(MoveErr("Invalid King move".to_owned()));
                    }
                }
                if KING_MOVE.contains(&diff)
                    && (is_empty(fen, to_move_index)
                        || dbg!(is_white(
                            dbg!(fen
                                .chars()
                                .nth(to_move_index as usize - 1)
                                .expect("OUT OF BOUNDS: King Move Check")), // make is in the opposite color from the moving piece
                        )) == dbg!(factor.is_positive())) // seems to work but not sure. TODO != (maybe?)
                {
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
        fen.chars()
            .nth(fen_idx(self.0) as usize - 1)
            .expect("OUT OF BOUNDS")
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
    pub fn rate_move_slight(&self) -> i32 {
        // simple evaluation
        // evaulate how good was the move based on the board
        unsafe {
            match TAKEN_PIECE {
                Some(x) => match x.to_ascii_uppercase() {
                    'P' => 1,
                    'N' => 3,
                    'B' => 3,
                    'R' => 5,
                    'Q' => 9,
                    _ => 0,
                },
                None => 0,
            }
        }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn rate_move(&self, fen: &String) -> i32 {
        // TODO
        // evaulate how good was the move based on the board
        // Will evaluate the board after the move is made
        // and take the difference

        todo!()
    }
}
/*
fn contigious_move_check(
    legal_moves: Vec<i8>,
    start: i8,
    diff: i8,
    fen: &String,
    factor: i8,
) -> bool {
    let piece_is_white = {
        let piece = fen
            .chars()
            .nth(start as usize)
            .expect("OUT OF BOUNDS: contigious_move_check");
        is_white(piece)
    };

    let mut result = dbg!(start);
    let to = dbg!(start + diff * factor);
    //let mut has_piece_between = false;
    for x in dbg!(legal_moves) {
        result += dbg!(x * factor); // TODO attempt to subtract with overflow >> 61 24
        for _ in 0..7 {
            // try to fix bug below by limiting the number of iterations (8 -> 7)
            dbg!(result);
            //if result < start && result > to && factor == -1{
            //return true;
            //}
            //if result > start && result < to && factor == 1 {
            //break;
            //}
            let index = result - 1;

            if index < 0 || index > 63 {
                break;
            } else {
                let piece = dbg!(fen
                    .chars()
                    .nth(index as usize)
                    .expect("OUT OF BOUNDS: contigious_move_check"));
                if dbg!(dbg!(piece_is_white) == dbg!(is_white(piece))) { // if there is a piece in
                                                                       // the way which is in the same
                                                                       // color
                    break; // > ADDED <
                           //has_piece_between = true;
                }
            if dbg!(result) == to { // changed -> && !has_piece_between
                return true;
            }
            result += dbg!(x * factor); // TODO attempt to subtract with overflow >> 61 24
            }
        }
        //has_piece_between = false;
        result = start;
    }
    false
}
*/
// TODO
pub fn is_white(piece: char) -> bool {
    piece.is_uppercase()
}

// TODO make this more efficient
//fn contigious_move_check(legal_moves: Vec<i8>, start:i8, diff: i8, fen: &String, factor: i8) -> bool {
//let mut result = dbg!(diff);
//let mut has_piece_between = false;
//for x in dbg!(legal_moves) {
//for _ in 0..7 {
//try to fix bug below by limiting the number of iterations (8 -> 7)
//dbg!(result);
//if !dbg!(has_piece_between) && dbg!(fen.chars().nth(dbg!(start * factor + result + 1) as usize).unwrap_or(' '))!= ' ' {
//has_piece_between = true;
//}
//if dbg!(result) == 0 && !has_piece_between {
//return true;
//}
//result += dbg!(x * factor); // TODO attempt to subtract with overflow >> 61 24
//}
//has_piece_between = false;
//result = diff;
//}
//false
//}

/* // Tried This:

fn contigious_move_check(legal_moves: Vec<i8>, start:i8, diff: i8, fen: &String, factor: i8) -> bool {
    let mut result = dbg!(diff);
    let mut has_piece_between = false;
    for x in dbg!(legal_moves) {
        for _ in 0..7 {
            // try to fix bug below by limiting the number of iterations (8 -> 7)
            dbg!(result);
            if !dbg!(has_piece_between) && dbg!(fen.chars().nth(dbg!(start + result * factor + 1 * factor) as usize).unwrap_or(' '))!= ' ' {
                has_piece_between = true;
            }
            if dbg!(result) == 0 && !has_piece_between {
                return true;
            }
            result += dbg!(x * factor); // TODO attempt to subtract with overflow >> 61 24
        }
        has_piece_between = false;
        result = diff;
    }
    false
}

*/

fn non_contigious_move_check(legal_moves: Vec<i8>, diff: i8) -> bool {
    legal_moves.iter().any(|&x| x == diff)
}

fn fen_idx(moved: [usize; 2]) -> i32 {
    // [x, y] -> (x + (y - 1) * 8)
    (moved[0] + (moved[1] - 1) * 8) as i32
}

fn contigious_move_check(
    legal_moves: Vec<i8>,
    start: i8,
    diff: i8,
    fen: &String,
    factor: i8,
) -> bool {
    let mut result = dbg!(start);
    let to = dbg!(start + diff * factor);
    let mut has_piece_between = false;
    for x in dbg!(legal_moves) {
        result += dbg!(x * factor); // TODO attempt to subtract with overflow >> 61 24
        for _ in 0..7 {
            // try to fix bug below by limiting the number of iterations (8 -> 7)
            dbg!(result);
            //if result < start && result > to && factor == -1{
            //return true;
            //}
            //if result > start && result < to && factor == 1 {
            //break;
            //}
            let index = result - 1;

            if index < 0 || index > 63 {
                break;
            } else {
                let piece = dbg!(fen
                    .chars()
                    .nth(result as usize - 1)
                    .expect("OUT OF BOUNDS: contigious_move_check"));
                if piece != ' ' && factor.is_negative() == is_white(piece) {
                    break; // > ADDED <
                           //has_piece_between = true;
                }
            }
            if dbg!(result) == to && !has_piece_between {
                return true;
            }
            result += dbg!(x * factor); // TODO attempt to subtract with overflow >> 61 24
        }
        has_piece_between = false;
        result = start;
    }
    false
}

// TODO
fn is_empty_till_n(fen: &String, to_move_index: i32, way: Offset, n: i8, turn: bool) -> bool {
    // Piece could be white or black
    // And piece can eat opponent's piece
    /* This algorithm first looks at the target
     * squareand loops till the piece which is
     * going tomove, if there is a non-empty
     * square, it returns false else true
     * * * * * * * * * * * * * * * * * * * * * */

    let mut inc = way.to_i32();
    let mut index = to_move_index;
    match turn {
        true => {
            index += 1;
            inc *= 1
        }
        false => {
            index -= 1;
            inc *= -1
        }
    }
    for _ in 0..n {
        // TODO out of bounds
        if fen.chars().nth(index as usize).unwrap_or('_') != ' ' {
            return false;
        }
        index += inc; // not checking current index (piece to be moved)
    }
    true
}

fn is_empty(fen: &String, to_move_index: i32) -> bool {
    !is_full(fen, to_move_index)
}

fn is_full(fen: &String, to_move_index: i32) -> bool {
    fen.chars()
        .nth(to_move_index as usize - 1)
        .unwrap_or(' ')
        //.expect("OUT OF BOUNDS:PAWN_CAPTURE")
        != ' '
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
    fn test_diff_fen() {
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
        assert_eq!(fen_idx([2, 1]), 2);
        assert_eq!(fen_idx([1, 3]), 17);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_general_game() {
        // TODO ADD SOME MOVES
        let mut board: Board = Default::default();
        let m = Move::new([5, 7], [5, 6]);
        board.move_piece(m.clone()).unwrap(); // This is testing purpuses only.
                                              // After piece moves, we don't
                                              // want to keep the old position
        assert_eq!(m.moved_piece(&board.FEN), 'P');
        board.draw(false);

        let m = Move::new([5, 2], [5, 4]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'p');
        board.draw(false);

        let m = Move::new([2, 8], [3, 6]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'N'); // has to be 57 but it is 58
        board.draw(false);

        let m = Move::new([2, 1], [3, 3]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'n');
        board.draw(false);

        let m = Move::new([6, 8], [3, 5]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'B');
        board.draw(false);

        let m = Move::new([6, 1], [2, 5]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'b');
        board.draw(false);

        let m = Move::new([5, 6], [5, 5]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'P');
        board.draw(false);

        let m = Move::new([5, 4], [5, 5]); // Errorenous move
        assert!(board.move_piece(m.clone()).is_err());

        let m = Move::new([5, 4], [5, 3]); // Errorenous move
        assert!(board.move_piece(m.clone()).is_err());

        let m = Move::new([5, 4], [5, 2]); // Errorenous move
        assert!(board.move_piece(m.clone()).is_err());

        let m = Move::new([7, 1], [7, 3]); // Errorenous move
        assert!(board.move_piece(m.clone()).is_err());

        let m = Move::new([7, 1], [8, 3]); // Errorenous move
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'n');

        /* uncomment this to see the steps of validation:
         * `cargo test` to see validation steps after uncommented*/
        //assert!(false == true); // This returns false so test will output stdout
    }

    #[test]
    fn test_pawn_move() {
        let mut board: Board = Default::default();
        let m = Move::new([5, 7], [5, 6]);
        let move_type = board.move_piece(m.clone()).unwrap();
        assert_eq!(move_type, MoveType::Regular);
        assert_eq!(m.moved_piece(&board.FEN), 'P');
    }

    #[test]
    fn test_pawn_capture() {
        let mut board: Board = Default::default();
        let m = Move::new([5, 7], [5, 5]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'P');
        board.draw(false);

        let m = Move::new([4, 2], [4, 4]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'p');
        board.draw(false);

        let m = Move::new([5, 5], [4, 4]); // Capture
        let move_type = board.move_piece(m.clone()).unwrap();
        assert_eq!(move_type, MoveType::PawnCapture);
    }

    //#[test]
    //fn test_pawn_double_move() { todo!() }

    //#[test]
    //fn test_pawn_blocked() { todo!() }

    //#[test]
    //fn test_pawn_promotion() { todo!() }

    #[test]
    fn test_knight_move() {
        let mut board: Board = Default::default();
        let m = Move::new([2, 8], [3, 6]);
        let move_type = board.move_piece(m.clone()).unwrap();
        assert_eq!(move_type, MoveType::Regular);
        assert_eq!(m.moved_piece(&board.FEN), 'N');
        board.draw(false);
    }

    #[test]
    fn test_knight_capture() {
        let mut board: Board = Default::default();
        let m = Move::new([7, 8], [6, 6]);
        board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'N');
        board.draw(false);

        let m = Move::new([5, 2], [5, 4]); // Pawn moves to knight
        board.move_piece(m.clone()).unwrap();
        board.draw(false);

        let m = Move::new([6, 6], [5, 4]); // Capture
        let move_type = board.move_piece(m.clone()).unwrap();
        assert_eq!(m.moved_piece(&board.FEN), 'N');
        assert_eq!(move_type, MoveType::Capture);
    }

    #[test]
    fn test_bishop_blocked() {
        let mut board: Board = Default::default();
        let m_white_bishop = Move::new([3, 8], [6, 5]);
        let move_type = board.move_piece(m_white_bishop.clone());
        assert!(move_type.is_err());
        board.draw(false); // TODO drawing first breakes to test
                           //assert_ne!(m.moved_piece(&board.FEN), 'B'); // TODO fix this

        let m_black_bishop = Move::new([3, 1], [6, 4]);
        let move_type = board.move_piece(m_black_bishop.clone());
        assert!(move_type.is_err());
        board.draw(false);
    }

    #[test]
    fn test_bishop_capture() {
        // This has to return an error because the bishop tries to eat a piece which is same color as itself
        let mut board: Board = Board::from_fen(
            "                      b                           p             ".to_owned(),
        );
        board.draw(false);
        let m_err = Move::new([7, 3], [3, 7]);
        let move_type_err = board.move_piece(m_err.clone());
        assert!(move_type_err.is_err());

        let mut board: Board = Board::from_fen(
            "                      B                           p             ".to_owned(),
        );
        let m_ok = Move::new([7, 3], [3, 7]);
        let move_type_ok = board.move_piece(m_ok.clone());
        assert!(move_type_ok.is_ok());
        assert_eq!(m_ok.moved_piece(&board.FEN), 'B');
        assert_eq!(move_type_ok.unwrap(), MoveType::Capture);
        board.draw(false);
    }

    #[test]
    fn test_rook_capture() {
        /* Forgot to reverse_turn! always white starts. shoot! */
        let mut board = Board::from_fen(
            "r      p                                                        ".to_owned(),
        );
        board.reverse_turn();
        let m_err = Move::new([1, 1], [8, 1]);
        let move_type_err = board.move_piece(m_err.clone());
        assert!(move_type_err.is_err());
        board.draw(false);

        let mut board = Board::from_fen(
            "r      P                                                        ".to_owned(),
        );
        board.reverse_turn();
        let m_ok = Move::new([1, 1], [8, 1]);
        let move_type_ok = board.move_piece(m_ok.clone()).unwrap();
        assert_eq!(m_ok.moved_piece(&board.FEN), 'r');
        assert_eq!(move_type_ok, MoveType::Capture);
        board.draw(false);
    }

    #[test]
    fn test_rook_move() {
        let mut board = Board::from_fen(
            "r                                                               ".to_owned(),
        );
        board.reverse_turn();
        let m_ok = Move::new([1, 1], [8, 1]);
        let move_type_ok = board.move_piece(m_ok.clone()).unwrap();
        assert_eq!(m_ok.moved_piece(&board.FEN), 'r');
        assert_eq!(move_type_ok, MoveType::Regular);
        board.draw(false);
    }

    // TODO add more comprehensive tests. (diagnal, horizontal, vertical)
    #[test]
    fn test_queen_move() {
        let mut board = Board::from_fen(
            "q                                                               ".to_owned(),
        );
        board.reverse_turn();
        let m_ok = Move::new([1, 1], [8, 1]);
        let move_type_ok = board.move_piece(m_ok.clone()).unwrap();
        assert_eq!(m_ok.moved_piece(&board.FEN), 'q');
        assert_eq!(move_type_ok, MoveType::Regular);
        board.draw(false);
    }

    #[test]
    fn test_queen_capture() {
        let mut board = Board::from_fen(
            "q      P                                                        ".to_owned(),
        );
        board.reverse_turn();
        let m_ok = Move::new([1, 1], [8, 1]);
        let move_type_ok = board.move_piece(m_ok.clone()).unwrap();
        assert_eq!(m_ok.moved_piece(&board.FEN), 'q');
        assert_eq!(move_type_ok, MoveType::Capture);
        board.draw(false);
    }

    #[test]
    fn test_king_move() {
        let mut board = Board::from_fen(
            "k                                                               ".to_owned(),
        );
        board.reverse_turn();
        let m_ok = Move::new([1, 1], [2, 1]);
        let move_type_ok = board.move_piece(m_ok.clone()).unwrap();
        assert_eq!(m_ok.moved_piece(&board.FEN), 'k');
        assert_eq!(move_type_ok, MoveType::Regular);
        board.draw(false);
    }

    #[test]
    fn test_king_capture() {
        let mut board = Board::from_fen(
            "kP                                                               ".to_owned(),
        );
        board.reverse_turn();
        let m_ok = Move::new([1, 1], [2, 1]);
        let move_type_ok = board.move_piece(m_ok.clone()).unwrap();
        assert_eq!(m_ok.moved_piece(&board.FEN), 'k');
        assert_eq!(move_type_ok, MoveType::Capture);
        board.draw(false);
    }

    #[test]
    fn test_king_castle() {}

    #[test]
    fn test_queen_castle() {}

    #[test]
    fn test_en_passant() {}

    #[test]
    fn test_promotion() {}

    #[test]
    fn test_check() {}

    #[test]
    fn test_checkmate() {}

    #[test]
    fn test_stalemate() {}

    #[test]
    fn test_invalid_move() {}

    #[test]
    fn test_insufficient_material() {}

    #[test]
    fn test_fifty_move_rule() {}

    #[test]
    fn test_threefold_repetition() {}

    #[test]
    fn test_draw() {}

    #[test]
    fn test_draw_by_agreement() {}

    #[test]
    fn test_win_by_resignation() {}
}
