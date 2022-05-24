pub struct Command {
    simple_commands: Vec<SimpleCommand>,
    outfile: String,
    infile: String,
    errfile: String,
    background: bool,
    append: bool,
}

pub struct SimpleCommand {
    args: Vec<String>,
}

impl Command {
    pub fn new() -> Command {
        Command {
            simple_commands: Vec::new(),
            outfile: String::new(),
            infile: String::new(),
            errfile: String::new(),
            background: false,
            append: false,
        }
    }
}

impl SimpleCommand {
    pub fn new() -> SimpleCommand {
        SimpleCommand { args: Vec::new() }
    }
}
