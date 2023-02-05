/* File: board.rs
 * Author: KoBruhh
 * Purpuse: Constructing a chess board virtually
 * Date: 05.02.2023
 * */

mod piece;
use piece::Piece::*;

const DEFAULT_PIECE_NOTATION: &str =
    "rnbqkbnr/pppppppp/        /        /        /        /PPPPPPPP/RNBQKBNR"; // I have chosen to use something called FEN to encode FEN into board. This is fixed sized.

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
    color: [(u8, u8, u8); 2], // Used only for displaying the board with OpenGL
    board: [[char; 8]; 8],    // Used for storing the board TODO
    FEN: String,              // Used for storing the FEN -> 72 is the max length of FEN
    turn: bool,               // White or black turn
    coordinates: bool,        // Used for displaying the coordinates
}

impl Board {
    pub fn new(color: [(u8, u8, u8); 2], coord: bool) -> Board {
        Board {
            color,
            board: [[' '; 8]; 8],
            FEN: String::from(DEFAULT_PIECE_NOTATION),
            turn: true,
            coordinates: coord,
        }
    }
    pub fn set_color(&mut self, color: [(u8, u8, u8); 2]) {
        self.color = color;
    }
    pub fn move_piece(&mut self, from: [usize; 2], to: [usize; 2]) {
        if from == to {
            panic!("You can't move a piece to the same place");
        }
        //if from or to includes any 0 or 9, then it's invalid
        for i in 0..2 {
            if from[i] < 1 || from[i] > 8 || to[i] < 1 || to[i] > 8 {
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
            const FRAME_VER: &str = "   |";
            const FRAME_HOR: &str = "-";
            const RTLB: &str = "|"; // Right Top Left Bottom
            const MARGIN: &str = " ";
            let size = 33;
            // Drawing board to the screen
            /* This is just experimental. I will convert this method to use OpenGL */
            let fen = &self.FEN;
            print!("{RTLB}");
            for piece in fen.chars() {
                match piece {
                    ' ' => print!("{MARGIN} {MARGIN}"),
                    'r' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Rook), //♜
                    'n' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Knight), //♞
                    'b' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Bishop), //♝
                    'q' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Queen), //♛
                    'k' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::King), //♚
                    'p' => print!("{MARGIN}{piece}{MARGIN}", piece = Black::Pawn), //♟
                    'R' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Rook), //♖
                    'N' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Knight), //♘
                    'B' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Bishop), //♗
                    'Q' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Queen), //♕
                    'K' => print!("{MARGIN}{piece}{MARGIN}", piece = White::King), //♔
                    'P' => print!("{MARGIN}{piece}{MARGIN}", piece = White::Pawn), //♙
                    '/' => print!("\n{}\n", FRAME_HOR.repeat(size)),
                    _ => panic!("Invalid FEN"),
                }
                print!("{RTLB}");
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
            if i != self.board.len() - 1 {
                tmp.push('/');
            }
        }
        self.FEN = tmp;
    }
    pub fn decode(&mut self) {
        // Will convert FEN to board
        let fen = &self.FEN.replace("/", "");
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.board[i][j] = fen.chars().nth(i * 8 + j).expect("Invalid FEN");
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut init = Board {
            color: [(0, 0, 0), (255, 255, 255)],
            board: [[' '; 8]; 8],
            FEN: String::from(DEFAULT_PIECE_NOTATION),
            turn: true, // white starts the game
            coordinates: true,
        };
        init.decode();
        init
    }
}
