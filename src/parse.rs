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
            Rule::io_modifier_list => {
                println!("{}", rule.as_str());
                if rule.as_str() != "" {
                    for io_modifier in rule.into_inner().next().unwrap().into_inner() {
                        let mut filename: String = String::new();
                        let io_modifier_clone = io_modifier.clone();
                        for tok in io_modifier_clone.into_inner() {
                            if tok.as_rule() == Rule::ident {
                                filename = String::from(tok.as_str());
                            }
                        }

                        match io_modifier.as_rule() {
                            Rule::output_modifier => {
                                command.outfile = filename;
                            }
                            Rule::input_modifier => {
                                command.infile = filename;
                            }
                            Rule::output_append_modifier => {
                                command.outfile = filename;
                                command.append = true;
                            }
                            Rule::error_output_modifier => {
                                command.errfile = filename;
                            }
                            Rule::error_append_modifier => {
                                command.errfile = filename;
                                command.append = true;
                            }
                            Rule::twogreat_modifier => {
                                command.append = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Rule::pipe_list => {
                if rule.as_str() != "" {
                    for pipe in rule.into_inner() {
                        for p in pipe.into_inner() {
                            if p.as_rule() == Rule::cmd_and_args {
                                command.add_simple_command(String::from(p.as_str()));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    command
}
