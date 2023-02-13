/* 
 * File: color.rs
 * Purpose: Contains the Color and BoardColor structs
 * Author: KoBruhh
 * Date: 11.02.2023
 * */

pub type BColor = (u8, u8, u8);
pub type BTheme = (BColor, BColor);

// Some Color Themes for the Board
#[allow(dead_code)]
pub mod themes {
    use super::BTheme;
    pub const COTTON_CANDY: BTheme = ((179, 154, 154), (186, 181, 171));
    pub const GRUVBOX: BTheme = ((104, 157, 106), (251, 241, 199));
    pub const GRUVBOX_DARK: BTheme = ((131, 148, 150), (40, 40, 40));
    pub const RUST: BTheme = ((219, 52, 0), (210, 191, 181));
    pub const BLANK: BTheme = ((0, 0, 0), (0, 0, 0));
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(u8, u8, u8);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoardColor(Color, Color);

impl Color {
    
    pub fn new(color: (u8, u8, u8)) -> Self {
        Color(color.0, color.1, color.2)
    }

    #[allow(dead_code)]
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

impl From<(u8, u8, u8)> for Color {
    
    fn from(rgb: (u8, u8, u8)) -> Self {
        Color(rgb.0, rgb.1, rgb.2)
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

impl From<BTheme> for BoardColor {
    
    fn from(theme: BTheme) -> Self {
        BoardColor::new(Color::new(theme.0), Color::new(theme.1))
    }

}
