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
    let mut parser = Parser::new();
    let mut file = File::create("output.asm").expect("Failed to create file");

        let mut code_writer = CodeWriter {
        label_id: 0,
        file_name: "Prog".to_string(),
    };
    // bootstrap
    writeln!(file, "@256").unwrap();
    writeln!(file, "D=A").unwrap();
    writeln!(file, "@SP").unwrap();
    writeln!(file, "M=D").unwrap();

    // call Sys.init 0
    for line in code_writer.write_call("Sys.init", 0) {
        writeln!(file, "{}", line).unwrap();
    }



    while parser.hasMoreLines() {
        parser.advance();
        if parser.current_cmd.starts_with("//") || parser.current_cmd.trim().is_empty() {
            continue;
        }
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
            parser::CommandType::FUNCTION => {
                instruction = code_writer.write_funciton(
                    parser.arg1(),
                    parser.arg2().expect(&format!(
                        "arg2 missing for function: {}.",
                        parser.current_cmd
                    )),
                );
            }
            parser::CommandType::CALL => {
                instruction = code_writer.write_call(
                    parser.arg1(),
                    parser
                        .arg2()
                        .expect(&format!("arg2 missing for call: {}.", parser.current_cmd)),
                );
            }
            parser::CommandType::RETURN => {
                instruction = code_writer.write_return();
            }

            parser::CommandType::LABEL => instruction = code_writer.write_label(parser.arg1()),
            parser::CommandType::GOTO => instruction = code_writer.write_goto(parser.arg1()),
            parser::CommandType::IF => instruction = code_writer.write_if(parser.arg1()),
            _ => println!("no matching parser command type in main"),
        }

        for line in instruction {
            writeln!(file, "{}", line).unwrap();
        }
    }
}
