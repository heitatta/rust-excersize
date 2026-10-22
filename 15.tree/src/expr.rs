use std::fmt::{Display, Formatter};

pub enum Expr {
    Number(i32),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

use Expr::*;

impl Expr {
    pub fn new_num(n: i32) -> Self {
        Number(n)
    }

    pub fn new_neg(e: Expr) -> Self {
        Neg(Box::new(e))
    }

    pub fn new_add(e1: Expr, e2: Expr) -> Self {
        Add(Box::new(e1), Box::new(e2))
    }

    pub fn new_mul(e1: Expr, e2: Expr) -> Self {
        Mul(Box::new(e1), Box::new(e2))
    }

    pub fn eval(&self) -> i32 {
        match self {
            Number(n) => *n,
            Neg(e) => - e.eval(),
            Add(e1, e2) => e1.eval() + e2.eval(),
            Mul(e1, e2) => e1.eval() * e2.eval(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "{}", n),
            Neg(e) => write!(f, "-({})", e),
            Add(e1, e2) => write!(f, "({} + {})", e1, e2),
            Mul(e1, e2) => write!(f, "({} * {})", e1, e2),
        }
    }
}
