use std::fmt::format;

use crate::parser::CommandType;

pub fn write_push_pop(command: CommandType, segment: &str, index: usize) {
    match command {
        CommandType::PUSH => {
            println!("pushing");
            write_push(command, segment, index);
        }
        CommandType::POP => {
            println!("popping");
            write_pop(command, segment, index);
        }
        _ => println!("invalid command"),
    }
}

pub fn write_arithmetic(command: &str) {
    let mut instructions: Vec<String> = vec![];
    match command {
        "add" => {
            instructions.push("@SP".to_string());
            instructions.push("A=M-1".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=A-1".to_string());
            instructions.push("D=D+M".to_string());
            instructions.push("M=D".to_string());
            instructions.push("@SP".to_string());
            instructions.push("M=M-1".to_string());
        }
        _ => println!("invalid command. No match in write arithmetic"),
    }
}

pub fn write_push(command: CommandType, segment: &str, index: usize) -> Vec<String> {
    // segment
    // index
    // stack
    let mut instructions: Vec<String> = vec![];

    match segment {
        "local" => {
            instructions.push("@LCL".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=M".to_string());
        }
        "argument" => {
            instructions.push("@ARG".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=M".to_string());
        }
        "this" => {
            instructions.push("@THIS".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=M".to_string());
        }
        "that" => {
            instructions.push("@THAT".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=M".to_string());
        }
        "temp" => {
            instructions.push("@TEMP".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=M".to_string());
        }
        "R13" => {
            instructions.push("@R13".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=M".to_string());
        }
        "R14" => {
            instructions.push("@R14".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=M".to_string());
        }
        "R15" => {
            instructions.push("@R15".to_string());
            instructions.push("D=M".to_string());
            instructions.push("A=M".to_string());
        }
        _ => println!("Invalid Segment. Segment did not match"),
    };

    // push index
    instructions.push(format!("@{}", index));
    instructions.push("A=D+A".to_string());
    instructions.push("D=M".to_string());

    // push push instrucions
    instructions.push("@SP".to_string());
    instructions.push("A=M".to_string());
    instructions.push("M=D".to_string());
    instructions.push("@SP".to_string());
    instructions.push("M=M+1".to_string());

    instructions
}

pub fn write_pop(command: CommandType, segment: &str, index: usize) -> Vec<String> {
    // get address
    //
    let mut instructions: Vec<String> = vec![];

    match segment {
        "local" => {
            instructions.push("@LCL".to_string());
            instructions.push("D=M".to_string());
        }
        "argument" => {
            instructions.push("@ARG".to_string());
            instructions.push("D=M".to_string());
        }
        "this" => {
            instructions.push("@THIS".to_string());
            instructions.push("D=M".to_string());
        }
        "that" => {
            instructions.push("@THAT".to_string());
            instructions.push("D=M".to_string());
        }
        "temp" => {
            instructions.push("@TEMP".to_string());
            instructions.push("D=M".to_string());
        }
        "R13" => {
            instructions.push("@R13".to_string());
            instructions.push("D=M".to_string());
        }
        "R14" => {
            instructions.push("@R14".to_string());
            instructions.push("D=M".to_string());
        }
        "R15" => {
            instructions.push("@R15".to_string());
            instructions.push("D=M".to_string());
        }
        _ => println!("Invalid Segment. Segment did not match"),
    };

    // Save address in R13
    instructions.push(format!("@{}", index));
    instructions.push("D=D+A".to_string());
    instructions.push("@R13".to_string());
    instructions.push("M=D".to_string());

    // pop stack
    instructions.push("@SP".to_string());
    instructions.push("A=M-1".to_string());
    instructions.push("D=M".to_string());
    instructions.push("@R13".to_string());
    instructions.push("A=M".to_string());
    instructions.push("M=D".to_string());
    instructions.push("@SP".to_string());
    instructions.push("M=M-1".to_string());

    instructions
}
