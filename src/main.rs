mod ast;
mod parser;

use std::str::from_utf8;
use ast::print::Printer;
use parser::Parser;

fn main() {
    let program = from_utf8(include_bytes!("test.an")).unwrap();
    let ast = Parser::new().parse(program);

    match ast {
        Ok(ref ast) => Printer::new().program(ast),
        Err(ref err) => println!("{}", err)
    }
}
