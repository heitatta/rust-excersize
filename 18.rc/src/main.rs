use std::rc::Rc;
use std::ops::Drop;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count = {}", Rc::strong_count(&a));
    }
    println!("count = {}", Rc::strong_count(&a));

    let d = Rc::new(Noisy("hello"));
    let e = Rc::clone(&d);
    let f = Rc::clone(&d);
    println!("hello = {}", Rc::strong_count(&f));
    drop(d);
    println!("hello = {}", Rc::strong_count(&f));
    drop(f);
    println!("hello = {}", Rc::strong_count(&e));

    let mut s = Rc::new(String::from("hello"));
    let _t = (*s).clone(); // これは String の clone であり、deep copy
    println!("string = {}", Rc::strong_count(&s)); // s の参照カウントは変わらない

    // Rc の参照カウントが 1 の場合のみ、内部の値を変更できる
    let mut_s = Rc::get_mut(&mut s);
    mut_s.unwrap().push_str(" world");
    println!("string = {}", s);

    let mut x = Rc::new(vec![1, 2, 3]);
    let y = Rc::clone(&x);
    Rc::make_mut(&mut x).push(4); // 複数カウントあるので、copy されたものが返っている
    println!("x = {:?}", x);
    println!("y = {:?}", y);
    drop(y); // y の参照カウントが 0 になるので、x の参照カウントは 1 になる
    Rc::make_mut(&mut x).push(5); // 今度は x がそのまま返っている
    println!("x = {:?}", x);

    //ex
    {
        
}


