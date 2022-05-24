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

		for rule in parsed.into_inner().next().unwrap().into_inner() {
			match rule.as_rule() {
				Rule::cmd_and_args => {
					for tok in rule.into_inner() {
						match tok.as_rule() {
							Rule::ident => cmd = String::from(tok.as_str()),
							Rule::arg => args.push(String::from(tok.as_str())),
							_ => {}
						}
					}
				}
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
