
use tree::List;
use List::{Cons, Nil};

use tree::Expr;
use tree::Node;

fn sum(l: &List) -> i32 {
    match l {
        Cons(head, tail) => head + sum(tail),
        Nil => 0,
    }
}

struct Noisy(&'static str);
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Drop {}", self.0);
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{}", list);
    println!("{}", sum(&list));
    println!("{}", size_of::<List>());

    // loop で length を書き換えたバージョン
    fn length(l: &List) -> usize {
        let mut len = 0;
        let mut current_list = l;
        loop {
            current_list = if let Cons(_, tail) = current_list { tail } else { break };
            len += 1;
        }
        len
    }
    println!("length of {} = {}", list, length(&list));

    let expr = Expr::new_add(
        Expr::new_mul(
            Expr::new_num(1),
            Expr::new_add(
                Expr::new_num(2),
                Expr::new_num(0),
            ),
        ),
        Expr::new_neg(
            Expr::new_neg(
                Expr::new_num(3),
            ),
        ),
    );
    println!("{}", expr);
    println!("{}", expr.eval());

    let mut n = Node::new(3);
    n.insert(2);
    n.insert(8);
    n.insert(5);
    n.insert(10);
    println!("{:?}", n);
    println!("contains(5)?: {}", n.contains(&5));

    let a = Box::new(Noisy("a"));
    let b = Box::new(Noisy("b"));
    let c = a;  // ここでは drop は走らない。move されるだけ
    println!("{} {}", c.0, b.0);
    println!("end");
    // ここで drop が走る
}
