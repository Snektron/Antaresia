extern crate phf;
extern crate phf_builder;

#[macro_use]
extern crate lazy_static;

mod ast;
mod parser;
mod check;

use std::str::from_utf8;
use ast::print::Printer;
use parser::Parser;
use parser::lexer::Lexer;

fn main() {
    let program = from_utf8(include_bytes!("test.an")).unwrap();
    let lexer = Lexer::new(program);
    let ast = Parser::new().parse(lexer);

    match ast {
        Ok(ref ast) => Printer::new().program(ast),
        Err(ref err) => println!("{}", err)
    }
}
