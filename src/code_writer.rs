use crate::parser::CommandType;

pub fn write_push_pop(cmd: CommandType) {
    match cmd {
        CommandType::PUSH => {
            println!("pushing");
            write_push();
        } // CommandType::POP => {
          //     println!("popping");
          //     write_pop();
          // }
    }
}

fn write_push(command: CommandType, segment: &str, index: usize) {
    let mut instructions: Vec<String>;

    match segment {
        "local" => {
            instructions.push("@LCL".to_string());
            instructions.push("A=M".to_string());
        }
        "argument" => {
            instructions.push("@ARG".to_string());
            instructions.push("A=M".to_string());
        }
        "this" => {
            instructions.push("@THIS".to_string());
            instructions.push("A=M".to_string());
        }
        "that" => {
            instructions.push("@THAT".to_string());
            instructions.push("A=M".to_string());
        }
        "temp" => {
            instructions.push("@TEMP".to_string());
            instructions.push("A=M".to_string());
        }
        "R13" => {
            instructions.push("@R13".to_string());
            instructions.push("A=M".to_string());
        }
        "R14" => {
            instructions.push("@R14".to_string());
            instructions.push("A=M".to_string());
        }
        "R15" => {
            instructions.push("@R15".to_string());
            instructions.push("A=M".to_string());
        }
    }
}
