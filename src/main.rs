extern crate phf;
extern crate phf_builder;

#[macro_use]
extern crate lazy_static;

mod ast;
mod parser;
mod check;
mod utility;

use std::str::from_utf8;
use parser::Parser;
use parser::lexer::Lexer;
use check::Context;
use ast::print::Printer;

fn main() {
    let program = from_utf8(include_bytes!("test.an")).unwrap();
    let lexer = Lexer::new(program);
    let ast = match Parser::new().parse(lexer) {
        Ok(ast) => ast,
        Err(err) => panic!("{}", err)
    };

    ast.print(&mut Printer::new());

    let ast = ast.check(&mut Context::new());

    println!("------");

    match ast {
        Ok(ast) => ast.print(&mut Printer::new()),
        Err(err) => println!("{}", err)
    }
}
