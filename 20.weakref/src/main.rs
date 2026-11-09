use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    name: &'static str,
    next: RefCell<Option<Rc<Node>>>
}
impl Drop for Node {
    fn drop(&mut self) {
        println!("  Drop {}", self.name);
    }
}

#[derive(Debug)]
struct Node2 {
    name: &'static str,
    next: RefCell<Weak<Node2>>,
}
impl Drop for Node2 {
    fn drop(&mut self) {
        println!("  Drop {}", self.name);
    }
}

// 練習問題
struct N(&'static str);
impl Drop for N {
    fn drop(&mut self) {
        println!("  Drop {}", self.0);
    }
}

fn main() {
    // 悪い例
    {
        let a = Rc::new(Node { name: "A", next: RefCell::new(None) });
        let b = Rc::new(Node { name: "B", next: RefCell::new(None) });
        *a.next.borrow_mut() = Some(b.clone());
        *b.next.borrow_mut() = Some(a.clone()); // これで循環参照を作れる
        println!("a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
        println!("b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
        // println!("{:?}", a); // これは stack overflow する。循環参照があるから。
    }
    // ここでメモリリークが起きている。a も b も drop されない。


    // weak ref を使った例
    {
        let b = Rc::new(Node2{
            name: "B",
            next: RefCell::new(Weak::new())
        });
        {
            let a = Rc::new(Node2{
                name: "A",
                next: RefCell::new(Weak::new())
            });
            *a.next.borrow_mut() = Rc::downgrade(&b);
            *b.next.borrow_mut() = Rc::downgrade(&a);
            println!("a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
            println!("b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
            println!("a: {:?}", a); // Weak のままで表示できる
        }
        // ここで a は drop され、b の weak ref も None になる
        println!("b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
        println!("b.next = {:?}", b.next.borrow().upgrade()); // a は drop されているので None になる
    }
    println!("End of the program!");

    // 練習問題
    {
        let a = Rc::new(N("a"));
        let w1 = Rc::downgrade(&a);
        let w2 = w1.clone();
        println!("{} {}", Rc::strong_count(&a), Rc::weak_count(&a));   // (1)
        let b = w2.upgrade();
        println!("{} {}", Rc::strong_count(&a), Rc::weak_count(&a));   // (2)
        drop(a);
        println!("{}", w1.upgrade().is_some());                         // (3)
        drop(b);
        println!("{}", w1.upgrade().is_some());                         // (4)
        println!("end");
    }
}
