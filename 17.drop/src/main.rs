use std::ops::Drop;

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

struct Outer {
    name: &'static str,
    inner1: Noisy,
    inner2: Noisy,
}

/// drop を利用したlock
struct Lock<'a> {
    flag: &'a mut bool,
    name: &'static str,
}
impl<'a> Lock<'a> {
    fn accuire(flag: &'a mut bool, name: &'static str) -> Option<Lock<'a>> {
        if *flag {
            None
        } else {
            *flag = true;
            println!("  lock {}", name);
            Some(Lock { flag, name })
        }
    }
}
impl Drop for Lock<'_> {
    fn drop(&mut self) {
        *self.flag = false;
        println!("  drop/unlock {}", self.name);
    }
}

fn sample1() -> Noisy {
    let a = Noisy("a");
    let _b = Noisy("b");
    let _c = Noisy("c");
    a
}

fn main() {
    let _a = sample1();
    let _ = Noisy("temp"); // これはその場で drop される
    let mut _x = Noisy("x");
    let _o = Outer {
        name: "o",
        inner1: Noisy("iner1"),
        inner2: Noisy("inner2"),
    };
    _x = Noisy("y"); // ここで Noisy("x") が drop される

    let early_drop = Noisy("early_drop");
    drop(early_drop); // 明示的に drop することもできる

    let mut flag = false;
    {
        let _l1 = Lock::accuire(&mut flag, "l1").unwrap();
        // let _l2 = Lock::accuire(&mut flag, "l1").unwrap(); // compile error 
    }
    // ブロックを抜けると取れる。
    Lock::accuire(&mut flag, "l1").unwrap();
    println!("main end");
    // 最後に _o, x, _a の順で drop される
}

