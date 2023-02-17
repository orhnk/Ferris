/*
 * File: commandline.rs
 * Purpose: Command line parsing and handling
 * Author: KoBruhh
 * Date: 11.02.2023
 */

static mut COMMANDS: Vec<Command> = Vec::new();

#[allow(dead_code)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub function: fn(),
}

impl Command {
    #[allow(dead_code)]
    pub fn new(name: &str, description: &str, function: fn()) {
        let command = Command {
            name: name.to_string(),
            description: description.to_string(),
            function: function,
        };
        unsafe {
            COMMANDS.push(command);
        }
    }

    #[allow(dead_code)]
    pub fn execute(&self) {
        (self.function)();
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}: {}", self.name, self.description);
    }

    #[allow(dead_code)]
    pub fn print_help(&self) {
        println!("{}: {}", self.name, self.description);
    }

    #[allow(dead_code)]
    pub fn print_help_all() {
        unsafe {
            for command in &COMMANDS {
                command.print_help();
            }
        }
    }

    #[allow(dead_code)]
    pub fn execute_command(name: &str) {
        unsafe {
            for command in &COMMANDS {
                if command.name == name {
                    command.execute();
                    return;
                }
            }
        }
        println!("Command not found");
    }
}

#[allow(dead_code)]
pub fn clear() {
    //print!("{}[2J", 27 as char);
    print!("{}", "\x1bc");
    //print!("{}", "\r".repeat(100)); // Didn't work TODO
}
