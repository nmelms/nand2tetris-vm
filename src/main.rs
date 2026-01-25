use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    code_writer::CodeWriter,
    parser::{CommandType, Parser},
};

mod code_writer;
mod parser;

fn main() {
    // read from the file
    // pasrse each line
    // get the command type
    // call fucntions with comamnds
    // read from file
    let mut parser = Parser::new();
    let mut file = File::create("output.asm").expect("Failed to create file");
    let mut code_writer = CodeWriter {
        label_id: 0,
        file_name: "Prog".to_string(),
    };

    while parser.hasMoreLines() {
        parser.advance();
        let mut instruction: Vec<String> = vec![];

        match parser.command_type() {
            parser::CommandType::PUSH | parser::CommandType::POP => {
                instruction = code_writer.write_push_pop(
                    parser.command_type(),
                    parser.arg1(),
                    parser.arg2().expect(&format!(
                        "arg2 missing for this command: {}.",
                        parser.current_cmd
                    )),
                )
            }
            parser::CommandType::ARITHMETIC => {
                instruction = code_writer.write_arithmetic(parser.arg1());
            }
            parser::CommandType::LABEL => instruction = code_writer.write_label(parser.arg1()),
            parser::CommandType::GOTO => instruction = code_writer.write_goto(parser.arg1()),
            parser::CommandType::IF => instruction = code_writer.write_if(parser.arg1()),
            _ => println!("no matching parser command type"),
        }

        for line in instruction {
            writeln!(file, "{}", line).unwrap();
        }
    }
}
