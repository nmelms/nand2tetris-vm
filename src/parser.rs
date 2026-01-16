pub struct Parser {
    lines: Vec<String>,
    i: usize,
    current_cmd: String,
    // split_current_cmd: Vec<&str>
}
#[derive(PartialEq)]
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
    pub fn new(&mut self) {
        self.lines = vec!["push lcoal 2".to_string()];
        self.i = 0;
        self.current_cmd = self.lines[self.i].clone();
        // self.split_current_cmd = self.lines[self.i].split(" ").collect();
    }

    pub fn hasMoreLines(&self) -> bool {
        if self.i < self.lines.len() {
            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) {
        self.current_cmd = self.lines[self.i].clone();
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

    pub fn arg1(&mut self) -> &str {
        let split_command: Vec<&str> = self.current_cmd.split(" ").collect();

        if self.command_type() != CommandType::RETURN {
            split_command[0]
        } else {
            "You cannot call arg1 on a RETURN type"
        }
    }

    pub fn arg2(&mut self) -> &str {
        let split_command: Vec<&str> = self.current_cmd.split(" ").collect();

        if self.command_type() == CommandType::PUSH
            || self.command_type() == CommandType::POP
            || self.command_type() == CommandType::FUNCTION
            || self.command_type() == CommandType::CALL
        {
            split_command[0]
        } else {
            "Arg2 needs to be called on a push, pop, function, or call"
        }
    }
}
