use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    code_writer::write_arithmetic,
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
    let mut file = File::create("output.txt").expect("Failed to create file");

    while parser.hasMoreLines() {
        let mut instruction: Vec<String> = vec![];

        match parser.command_type() {
            parser::CommandType::PUSH | parser::CommandType::POP => {
                instruction = code_writer::write_push_pop(
                    parser.command_type(),
                    parser.arg1(),
                    parser.arg2().unwrap(),
                )
            }
            parser::CommandType::ARITHMETIC => {
                instruction = code_writer::write_arithmetic(parser.arg1());
            }
            _ => println!("no matching parser command type"),
        }

        for line in instruction {
            writeln!(file, "{}", line).unwrap();
        }

        parser.advance();
    }
}
