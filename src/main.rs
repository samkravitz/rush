use std::io::{stdin, stdout, Write};

fn main() {
	loop {
		prompt();
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
	}
}

fn prompt() {
	print!("> ");
	stdout().flush().ok();
}
