extern crate phf;
extern crate phf_builder;

#[macro_use]
extern crate lazy_static;

mod ast;
mod parser;

use parser::Parser;
use ast::print::Printer;

fn main() {
    let program = include_bytes!("test.an");
    let mut p = Parser::new(program.as_ref());
    let tree = p.program();

    match tree {
        Ok(ref tree) => Printer::new().program(tree),
        Err(ref err) => println!("Error: {}", err)
    };
}
