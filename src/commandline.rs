/*
 * File: commandline.rs
 * Purpose: Command line parsing and handling
 * Author: KoBruhh
 * Date: 11.02.2023
 */
use std::process::Command;

pub fn clear() {
    print!("{}[2J", 27 as char);
    //print!("{}", "\r".repeat(100)); // Didn't work TODO
}
