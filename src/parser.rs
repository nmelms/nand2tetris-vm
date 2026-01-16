pub struct Parser {
    lines: Vec<String>,
    i: usize,
    current_cmd: String,
}

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

    pub fn command_type(&self) {
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
        };
    }
}
