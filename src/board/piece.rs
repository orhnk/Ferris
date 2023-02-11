/* File: piece.rs
 * Purpuse: representing every chess piece on the board with data structures
 * Author: KoBruhh
 * Date: 06.02.2023
 * Note: This File did not worked very well because now board is dependant to FEN notation.
 * I don't know if it is neccesary or not.
 * */

use std::fmt::{Display, Formatter};
pub enum White {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
pub enum Black {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum Piece {
    White(White),
    Black(Black),
    Void,
}

impl Piece {
    pub fn is_white(&self) -> bool {
        match self {
            Piece::White(_) => true,
            _ => false,
        }
    }
    pub fn is_black(&self) -> bool {
        match self {
            Piece::Black(_) => true,
            _ => false,
        }
    }
    pub fn is_void(&self) -> bool {
        match self {
            Piece::Void => true,
            _ => false,
        }
    }
}

pub fn char_to_piece(c: char) -> Piece {
    match c {
        'P' => Piece::White(White::Pawn),
        'N' => Piece::White(White::Knight),
        'B' => Piece::White(White::Bishop),
        'R' => Piece::White(White::Rook),
        'Q' => Piece::White(White::Queen),
        'K' => Piece::White(White::King),
        'p' => Piece::Black(Black::Pawn),
        'n' => Piece::Black(Black::Knight),
        'b' => Piece::Black(Black::Bishop),
        'r' => Piece::Black(Black::Rook),
        'q' => Piece::Black(Black::Queen),
        'k' => Piece::Black(Black::King),
        _ => Piece::Void,
    }
}

impl Display for White {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            White::Pawn => write!(f, "P"),
            White::Knight => write!(f, "N"),
            White::Bishop => write!(f, "B"),
            White::Rook => write!(f, "R"),
            White::Queen => write!(f, "Q"),
            White::King => write!(f, "K"),
        }
    }
}
impl Display for Black {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Black::Pawn => write!(f, "p"),
            Black::Knight => write!(f, "n"),
            Black::Bishop => write!(f, "b"),
            Black::Rook => write!(f, "r"),
            Black::Queen => write!(f, "q"),
            Black::King => write!(f, "k"),
        }
    }
}
