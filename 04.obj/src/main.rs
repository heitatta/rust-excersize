struct Counter {
    name: String,
    count: u32,
}

impl Counter {
    fn new(name: &str) -> Counter {
        Counter { name: name.to_string(), count: 0 }
    }
    fn increment(&mut self) { self.count += 1; }   // 更新する
    fn reset(&mut self) { self.count = 0; }          // 更新する
    fn is_zero(&self) -> bool { self.count == 0 }   // 読むだけ
    fn count(&self) -> u32 { self.count }          // 読むだけ
    fn name(&self) -> &str { &self.name }          // 読むだけ（参照を返す）
    fn finish(self) -> String {                    // 所有権を消費する
        format!("{}: {}", self.name, self.count)
    }
}

fn main() {
    let mut c = Counter::new("hits");   // 更新するので mut
    println!("Is c zero?: {}", c.is_zero());   // true
    c.increment();
    c.increment();
    println!("{} = {}", c.name(), c.count());   // hits = 2
    println!("Is c zero?: {}", c.is_zero());   // true
    c.reset();
    println!("Is c zero?: {}", c.is_zero());   // true
    let summary = c.finish();           // c の所有権が消費される
    println!("{}", summary);            // hits: 2

    let mut v = [Counter::new("a"),
                 Counter::new("b"),
                 Counter::new("c")];
    for c in v.iter_mut() {   // 更新するので iter_mut
        c.increment();
        println!("{} = {}", c.name(), c.count());
    }
    for c in v.iter() {
        // c.increment();  // v.ter() は &v と同じ意味。Counter を返すので、更新できない
        println!("{} = {}", c.name(), c.count());
    }
}

