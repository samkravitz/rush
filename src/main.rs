use std::io::{stdin, stdout, Write};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rush.pest"]
pub struct PestParser;

fn main() {
	loop {
		prompt();
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();

		let parsed = PestParser::parse(Rule::main, line.trim())
			.expect("Unsuccessful parse")
			.next()
			.unwrap();

		let mut cmd: String = String::new();
		let mut args: Vec<String> = Vec::new();

		for tok in parsed.into_inner() {
			match tok.as_rule() {
				Rule::cmd => cmd = String::from(tok.as_str()),
				Rule::arg => args.push(String::from(tok.as_str())),
				_ => {}
			}
		}

		println!("{} {:?}", cmd, args);
	}
}

fn prompt() {
	print!("> ");
	stdout().flush().ok();
}
