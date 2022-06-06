#[macro_use]
extern crate pest_derive;

mod command;
mod parse;

use std::io::{stdin, stdout, Write};
use std::process::{Command, Stdio};

fn main() {
    loop {
        prompt();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let command = parse::parse(line.trim());

        let num_commands = command.simple_commands.len();
        if num_commands == 0 {
            continue;
        }

        let mut prev_command = None;
        for i in 0..num_commands {
            let simple_command = &command.simple_commands[i];
            let is_final_command = i == num_commands - 1;

            let stdin = prev_command.map_or(Stdio::inherit(), |output: std::process::Child| {
                Stdio::from(output.stdout.unwrap())
            });

            let stdout = if is_final_command {
                Stdio::inherit()
            } else {
                Stdio::piped()
            };

            let child = Command::new(&simple_command.cmd)
                .args(&simple_command.args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn();

            prev_command = match child {
                Ok(prev) => Some(prev),
                Err(_) => {
                    println!("[rush]: Unknown command: {}", &simple_command.cmd);
                    None
                }
            };
        }

        if let Some(mut final_command) = prev_command {
            final_command.wait().ok();
        }
    }
}

fn prompt() {
    print!("> ");
    stdout().flush().ok();
}
