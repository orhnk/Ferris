pub struct Color(u8, u8, u8);
pub struct BoardColor(Color, Color);

impl Color {
    
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    pub fn foreground(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.0, self.1, self.2)
    }
    pub fn background(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.0, self.1, self.2)
    }
}

impl BoardColor {
    
    pub fn new(foreground: Color, background: Color) -> Self {
        BoardColor(foreground, background)
    }

    pub fn rgb(&self) -> (&Color, &Color) {
        (&self.0, &self.1)
    }

}

impl Default for BoardColor {
    
    fn default() -> Self {
        BoardColor(Color::new(168, 243, 218), Color::new(147, 122, 183))
    }

}
