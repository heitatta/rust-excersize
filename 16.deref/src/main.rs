use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let r = &x;
    println!("{}", *r+1); // r+1 でも同じ。自動deref

    let b = Box::new(5);
    println!("{}", *b+1); // Box も同じ書き方。b+1 とは書けない

    let m = MyBox(5);
    println!("{}", *m+1); // Deref を impl すれば、*m で *(m.deref())
                          // が呼ばれるようになり、値を参照できる。

    let name = MyBox(String::from("Rust"));
    hello(&name); // これで name.deref().deref() (MyBox->String->&str)される

    // 関数呼び出し時にも自動 deref が働く
    println!("{} {}", name.len(), name.to_uppercase());

    let bb = Box::new(MyBox(String::from("Rust")));
    println!("{} {}", bb.len(), bb.to_uppercase()); // Box->MyBox->String->&str
    // deref_mut も同様に働く
    let mut m = MyBox(vec![1, 2, 3]);
    m.push(30); // &mut self が必要なメソッドを呼び出すと、deref_mut() が自動で呼ばれる
    println!("{:?}", *m);
    *m = vec![0]; // 代入も deref_mut() が呼ばれる
    println!("{:?}", *m);

    ex();
}

fn greet(s: &str) { println!("Hello, {}!", s); }
fn ex() {
    let b = Box::new(String::from("a"));
    let rc = Box::new(b);
    greet(&rc);
    let n = Box::new(5);
    let _m = *n + 1;

    let w = MyBox(String::from("mybox"));
    let w_ref = &w;
    println!("{}", w_ref.len());
    println!("{}", (*w_ref).len());
}
