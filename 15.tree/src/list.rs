use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum List {
    // Cons(i32, List),   // error[E0072]: recursive type `List` has infinite size
    Cons(i32, Box<List>), // Box でヒープに置くことで、サイズを固定する
    Nil,
}

use List::{Cons, Nil};

impl List {
    pub fn new() -> Self {
        Nil
    }

    pub fn car(self) -> Option<i32> {
        match self {
            Cons(head, _) => Some(head),
            Nil => None,
        }
    }

    pub fn cdr(self) -> List {
        match self {
            Cons(_, tail) => *tail,
            Nil => Nil,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Cons(_, _) => false,
            Nil => true,
        }
    }
}

impl Default for List {
    fn default() -> Self {
        List::new()
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cons(head, tail) => write!(f, "Cons({}, {})", head, tail),
            Nil => write!(f, "Nil")
        }
    }
}
