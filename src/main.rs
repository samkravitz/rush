#[macro_use]
extern crate pest_derive;

mod command;
mod parse;

use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        prompt();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let command = parse::parse(line.trim());
        for simple_command in command.simple_commands {
            let result = std::process::Command::new(&simple_command.cmd)
                .args(&simple_command.args)
                .spawn();
            if let Ok(mut child) = result {
                child.wait().ok();
            } else {
                println!("[rush]: Unknown command: {}", &simple_command.cmd);
            }
        }
    }
}

fn prompt() {
    print!("> ");
    stdout().flush().ok();
}
