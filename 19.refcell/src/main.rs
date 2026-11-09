use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let _x = 5;
    // let y = &mut _x; // compile error[E0596]: cannot borrow `x` as mutable
    let c = RefCell::new(5); // mut 変数ではないが、
    println!("c = {}", c.borrow());
    {
        *c.borrow_mut() += 1; // 中身を書き替えられる。これはコンパイルを通る
        println!("c = {}", c.borrow());
    }
    // 上の block は抜けているので、ここで borrow してもコンパイルエラーにはならない
    let _r = c.borrow();
    // let _w = *c.borrow_mut() += 1; // コンパイルは通るが、panic する(RefCell already borrowed)

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    *value.borrow_mut() += 10; // 共有可変変数を書き換える
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    // Copy 可能な型なら、Cell が使える
    let c = Cell::new(5);
    println!("c = {}", c.get());
    c.set(c.get() + 1);
    println!("c = {}", c.get());
    let _ = c.replace(10); // 返り値は古い値
    println!("c = {}", c.get());

    // cell を使ったcounterの例。c1 も c2 も同時に更新される
    let counter = Cell::new(0);
    let c1 = &counter;
    let c2 = &counter;
    println!("c1 = {}, c2 = {}", c1.get(), c2.get());
    c1.set(c1.get() + 1);
    println!("c1 = {}, c2 = {}", c1.get(), c2.get());
    c2.set(c2.get() + 1);
    println!("c1 = {}, c2 = {}", c1.get(), c2.get());
}
