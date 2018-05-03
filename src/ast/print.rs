use std::convert::AsRef;
use std::cmp;
use ast::{Program, Stmt, StmtKind, Expr, ExprKind};
use check::CheckType;

pub struct Printer {
    stack: Vec<usize>
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            stack: Vec::new()
        }
    }

    fn leaf<S>(&mut self, text: S)
    where S: AsRef<str> {
        self.node(0, text)
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
}

impl<C> Program<C>
where C: CheckType {
    pub fn print(&self, p: &mut Printer) {
        p.node(self.stmts.len(), format!("Program(#stmts = {})", self.stmts.len()));

        for child in self.stmts.iter() {
            child.print(p);
        }
    }
}

impl<C> Stmt<C>
where C: CheckType {
    pub fn print(&self, p: &mut Printer) {
        match self.kind {
            StmtKind::Compound(ref children) => {
                p.node(children.len(), format!("Compound(#stmts = {})", children.len()));

                for child in children {
                    child.print(p);
                }
            },
            StmtKind::If(ref cond, ref cons, None) => {
                p.node(2, "If");
                cond.print(p);
                cons.print(p);
            },
            StmtKind::If(ref cond, ref cons, Some(ref alt)) => {
                p.node(3, "IfElse");
                cond.print(p);
                cons.print(p);
                alt.print(p);
            },
            StmtKind::While(ref cond, ref cons) => {
                p.node(2, "While");
                cond.print(p);
                cons.print(p);
            },
            StmtKind::Return(ref expr) => {
                p.node(1, "Return");
                expr.print(p);
            },
            StmtKind::Expr(ref expr) => {
                p.node(1, "Expr");
                expr.print(p);
            },
            StmtKind::FuncDecl(ref decl) => {
                p.node(1 + decl.params.len(), format!("FuncDecl({} -> {})", decl.name, decl.return_ty));

                for param in decl.params.iter() {
                    p.leaf(format!("Param({})", param));
                }

                decl.body.print(p);
            },
            StmtKind::StructDecl(ref name, ref fields) => {
                p.node(fields.len(), format!("StructDecl({})", name));

                for field in fields {
                    p.leaf(format!("Member({})", field));
                }
            }
        }   
    }
}

impl<C> Expr<C>
where C: CheckType {
    pub fn print(&self, p: &mut Printer) {
        match self.kind {
            ExprKind::Binary(ref op, ref lhs, ref rhs) => {
                p.node(2, format!("Binary(op = {:?})", op));
                lhs.print(p);
                rhs.print(p);
            },
            ExprKind::Unary(ref op, ref rhs) => {
                p.node(1, format!("Unary(op = {:?})", op));
                rhs.print(p);
            },
            ExprKind::Call(ref callee, ref args) => {
                p.node(args.len() + 1, "Call");
                callee.print(p);

                for arg in args {
                    arg.print(p);
                }
            },
            ExprKind::Subscript(ref lhs, ref rhs) => {
                p.node(2, "Subscript");
                lhs.print(p);
                rhs.print(p);
            },
            ExprKind::Cast(ref lhs, ref dt) => {
                p.node(1, format!("Cast({})", dt));
                lhs.print(p);
            },
            ExprKind::ImplicitCast(ref lhs, ref dt) => {
                p.node(1, format!("ImplicitCast({})", dt));
                lhs.print(p);
            },
            ExprKind::Literal(ref x) => p.leaf(format!("Literal({:?})", x)),
            ExprKind::Name(ref name) => p.leaf(format!("Name(name = {})", name)),
            ExprKind::Decl(ref field, ref val) => {
                let text = format!("Decl({})", field);
                match *val {
                    Some(ref expr) => {
                        p.node(1, text);
                        expr.print(p);
                    },
                    None => p.leaf(text)
                }
            }
        }
    }
}
