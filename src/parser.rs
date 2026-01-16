

pub struct Parser{
    lines: Vec<String>,
    i: usize,
    current_cmd: String,
}

pub enum CommandType{
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
        }else{
            false
        }
    }


    pub fn advance(&mut self){
        self.current_cmd = self.lines[self.i].clone();
        self.i += 1;
    }

    pub fn command_type(&self){
        let split_command: Vec<&str> = self.current_cmd.spit(" ");

        match split_command[0]{
            "add" => CommandType::ARITHMETIC,
            _ => CommandType::NONE,
        };
    }   


}