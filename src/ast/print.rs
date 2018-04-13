use std::convert::AsRef;
use std::cmp;
use ast::{Expr, Stmt, Program};

pub struct Printer {
    stack: Vec<usize>
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            stack: Vec::new()
        }
    }

    fn node<S>(&mut self, children: usize, text: S)
    where S: AsRef<str> {
        let mut buf = String::new();
        for i in self.stack.iter().take(cmp::max(self.stack.len(), 1) - 1) {
            if *i > 0 {
                buf.push_str("| ");
            } else {
                buf.push_str("  ");
            }
        }

        if let Some(x) = self.stack.last() {
            if *x > 0 {
                buf.push_str("|-");
            } else {
                buf.push_str("`-");
            }
        }

        println!("{}{}", buf, text.as_ref());

        if children > 0 {
            self.stack.push(children - 1);
        } else {
            while self.stack.len() > 0 && self.stack.last().unwrap() <= &0 {
                self.stack.pop();
            }

            if self.stack.len() > 0 {
                *self.stack.last_mut().unwrap() -= 1;
            }
        }
    }

    pub fn program(&mut self, program: &Program) {
        self.node(program.0.len(), format!("Program(#stmts = {})", program.0.len()));
        for child in program.0.iter() {
            self.stmt(child);
        }
    }

    pub fn stmt(&mut self, stmt: &Stmt) {
        match *stmt {
            Stmt::Compound(ref children) => {
                self.node(children.len(), format!("Compound(#stmts = {})", children.len()));

                for child in children {
                    self.stmt(child);
                }
            },
            Stmt::If(ref cond, ref cons, None) => {
                self.node(2, "If");
                self.expr(cond);
                self.stmt(cons);
            },
            Stmt::If(ref cond, ref cons, Some(ref alt)) => {
                self.node(3, "IfElse");
                self.expr(cond);
                self.stmt(cons);
                self.stmt(alt);
            },
            Stmt::While(ref cond, ref cons) => {
                self.node(2, "While");
                self.expr(cond);
                self.stmt(cons);
            },
            Stmt::Return(ref expr) => {
                self.node(1, "Return");
                self.expr(expr);
            },
            Stmt::Expr(ref expr) => {
                self.node(1, "Expr");
                self.expr(expr);
            },
            Stmt::FuncDecl(ref name, ref rtype, ref params, ref body) => {
                self.node(1, format!("FuncDecl(name = {}, return type = {:?}, params = {:?})", name, rtype, params));
                self.stmt(body);
            },
            Stmt::StructDecl(ref name, ref fields) => {
                self.node(0, format!("StructDecl(name = {:?}, fields = {:?})", name, fields));
            }
        }        
    }

    pub fn expr(&mut self, expr: &Expr) {
        match *expr {
            Expr::Binary(ref op, ref lhs, ref rhs) => {
                self.node(2, format!("Binary(op = {:?})", op));
                self.expr(lhs);
                self.expr(rhs);
            },
            Expr::Unary(ref op, ref rhs) => {
                self.node(1, format!("Unary(op = {:?})", op));
                self.expr(rhs);
            },
            Expr::Call(ref callee, ref args) => {
                self.node(args.len() + 1, "Call");
                self.expr(callee);

                for arg in args {
                    self.expr(arg);
                }
            },
            Expr::Subscript(ref lhs, ref rhs) => {
                self.node(2, "Subscript");
                self.expr(lhs);
                self.expr(rhs);
            },
            Expr::Literal(ref x) => self.node(0, format!("Literal(value = {:?})", x)),
            Expr::Name(ref name) => self.node(0, format!("Name(name = {})", name)),
            Expr::Decl(ref dt, ref name, ref val) => {
                self.node(if val.is_some() { 1 } else { 0 }, format!("Decl(type = {:?}, name = {})", dt, name));
                if val.is_some() {
                    self.expr(val.as_ref().unwrap());
                }
            }
        }
    }
}