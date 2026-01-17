mod code_writer;
mod parser;

fn main() {
    let res = code_writer::write_push(parser::CommandType::PUSH, "local", 2);

    println!("{:?}", res);
}
