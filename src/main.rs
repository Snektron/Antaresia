extern crate phf;
extern crate phf_builder;

#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod parser;
pub mod check;
pub mod utility;

use std::str::from_utf8;
use parser::Parser;
use parser::lexer::Lexer;
use ast::print::Printer;
use check::typecheck::Checker;

fn main() {
    let program = from_utf8(include_bytes!("test.an")).unwrap();
    let lexer = Lexer::new(program);
    let ast = match Parser::new().parse(lexer) {
        Ok(ast) => ast,
        Err(err) => panic!("{}", err)
    };

    ast.print(&mut Printer::new());

    let mut checker = Checker::new();
    let ast = checker.check_program(ast);

    println!("------");

    match ast {
        Ok(ast) => ast.print(&mut Printer::new()),
        Err(err) => println!("{}", err)
    }
}
