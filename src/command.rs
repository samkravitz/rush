#[derive(Debug)]
pub struct Command {
    pub simple_commands: Vec<SimpleCommand>,
    pub outfile: String,
    pub infile: String,
    pub errfile: String,
    pub background: bool,
    pub append: bool,
}

#[derive(Debug)]
pub struct SimpleCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(
        outfile: String,
        infile: String,
        errfile: String,
        background: bool,
        append: bool,
    ) -> Command {
        Command {
            simple_commands: Vec::new(),
            outfile,
            infile,
            errfile,
            background,
            append,
        }
    }

    pub fn add_simple_command(&mut self, s: String) {
        self.simple_commands.push(SimpleCommand::new(s));
    }
}

impl SimpleCommand {
    pub fn new(s: String) -> SimpleCommand {
        let mut args: Vec<String> = Vec::new();
        let split: Vec<&str> = s.split(" ").collect();
        let cmd = String::from(split[0]);

        for arg in &split[1..] {
            args.push(String::from(*arg));
        }
        SimpleCommand { cmd, args }
    }
}
