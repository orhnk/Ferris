/* File: parser.rs
 * Purpuse: Alowing taking a user input and converting it to coordinates.
 * eg. e4 -> [5, 7], [5, 5] (from, to) coordinates. (<< I will do this later 
 * because it is harder compared to simple coordinates)
 * Author: KoBruhh
 * Date: 05.02.2023
 * */

use std::{error::Error, fmt::{Display}};

//const PARSE_ERR:&str = r#"Invalid input! Expected integers as input! (Spaces are ignored)"#;

#[derive(Debug)]
pub struct ParseErr;

impl Error for ParseErr {}
impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseErr: Failed to parse input!")
    }
}

pub fn convert_to_coords(usr: &str) -> Result<[[usize;2]; 2], ParseErr> {
    let usr = usr.replace(" ", "");
    let usr = usr.trim();
    if usr.len() != 4 {
        return Err(ParseErr);
    }
    let mut result = [[0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            if let Some(num) = usr.chars().nth(i*2+j).unwrap().to_digit(10) {
                result[i][j] = num as usize;
            } else {
                return Err(ParseErr);
            }
        }
    }
    
    Ok(result)
}
