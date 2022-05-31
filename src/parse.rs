use crate::command;

extern crate pest;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rush.pest"]
pub struct PestParser;

pub fn parse(input: &str) -> command::Command {
    let parsed = PestParser::parse(Rule::main, input)
        .expect("Unsuccessful parse")
        .next()
        .unwrap();

    let mut command =
        command::Command::new(String::new(), String::new(), String::new(), false, false);

    for rule in parsed.into_inner().next().unwrap().into_inner() {
        match rule.as_rule() {
            Rule::cmd_and_args => {
                command.add_simple_command(String::from(rule.as_str()));
            }
            _ => {}
        }
    }

    command
}
