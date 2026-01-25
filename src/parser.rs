use std::fs;

pub struct Parser {
    lines: Vec<String>,
    i: usize,
    pub current_cmd: String,
    // split_current_cmd: Vec<&str>
}
#[derive(PartialEq, Debug)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
    NONE,
}

impl Parser {
    pub fn new() -> Self {
        let file_contents = fs::read_to_string("Prog.vm").expect("failed to read VM file");
        let lines: Vec<String> = file_contents.lines().map(String::from).collect();
        let i = 0;
        let current_cmd = lines[i].trim().to_string().clone();
        // self.split_current_cmd = self.lines[self.i].split(" ").collect();

        Self {
            lines,
            i,
            current_cmd,
        }
    }

    pub fn hasMoreLines(&mut self) -> bool {
        if self.i < self.lines.len() {
            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) {
        self.current_cmd = self.lines[self.i].trim().to_string().clone();
        self.i += 1;
    }

    pub fn command_type(&self) -> CommandType {
        let split_command: Vec<&str> = self.current_cmd.split(" ").collect();

        match split_command[0] {
            "add" => CommandType::ARITHMETIC,
            "sub" => CommandType::ARITHMETIC,
            "neg" => CommandType::ARITHMETIC,
            "eq" => CommandType::ARITHMETIC,
            "gt" => CommandType::ARITHMETIC,
            "lt" => CommandType::ARITHMETIC,
            "and" => CommandType::ARITHMETIC,
            "or" => CommandType::ARITHMETIC,
            "not" => CommandType::ARITHMETIC,
            "push" => CommandType::PUSH,
            "pop" => CommandType::POP,
            "label" => CommandType::LABEL,
            "goto" => CommandType::GOTO,
            "if-goto" => CommandType::IF,
            "function" => CommandType::FUNCTION,
            "return" => CommandType::RETURN,
            "call" => CommandType::CALL,
            _ => CommandType::NONE,
        }
    }

    pub fn arg1(&self) -> &str {
        let split_command: Vec<&str> = self.current_cmd.split(" ").collect();

        if self.command_type() == CommandType::ARITHMETIC {
            split_command[0]
        } else if self.command_type() != CommandType::RETURN {
            // should probably refactor this to get
            split_command[1]
        } else {
            "You cannot call arg1 on a RETURN type"
        }
    }

    pub fn arg2(&self) -> Option<usize> {
        let split_command: Vec<&str> = self.current_cmd.split(" ").collect();
        println!("inside arg 2: {:?}", split_command);

        if self.command_type() == CommandType::PUSH
            || self.command_type() == CommandType::POP
            || self.command_type() == CommandType::FUNCTION
            || self.command_type() == CommandType::CALL
        {
            split_command.get(2)?.parse::<usize>().ok()
        } else {
            None
        }
    }
}
