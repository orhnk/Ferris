
pub struct Move([usize; 2], [usize; 2]); // start(x, y), end(x, y)

impl Move {
    pub fn new(start: [usize; 2], end: [usize; 2]) -> Self {
        Move(start, end)
    }
    pub fn decode_move(&self) -> ([usize; 2], [usize; 2]) {
        (self.0, self.1)
    }
    pub fn validate_move(&self) {
        todo!();
    }
}


