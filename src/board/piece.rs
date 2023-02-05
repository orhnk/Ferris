pub mod Piece {
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
}
