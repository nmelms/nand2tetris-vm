mod code_writer;
mod parser;

fn main() {
    let res = code_writer::write_pop(parser::CommandType::POP, "local", 2);

    println!("{:?}", res);
}
