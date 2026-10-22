/// 無限リストを使って実現する Fibonacci 数列の例
struct Fibbonacci {
    a: u64,
    b: u64
}

impl Fibbonacci {
    fn new() -> Self {
        Fibbonacci { a: 1, b: 1 }
    }
}
impl Iterator for Fibbonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let r = self.a;
        self.a = self.b;
        self.b += r;
        Some(r)
    }
}

fn main() {
    // map, filter, take, zip, rev, skip など「アダプタ」と呼ばれるメソッドは遅延評価。必要になるまで計算されない
    let lazy = (1..=3).map(|x| { println!("  map({})", x); x * 10 });
    let filtered = lazy.filter(|x| { println!("  filter({})", x); *x != 20 });
    println!("アダプタを作っただけ。まだ何も実行されていない");
    filtered.for_each(|x| println!("{}", x));

    // 無限リストの例
    println!("{:?}", (1..).take(5).collect::<Vec<_>>());
    println!("{:?}", (1..).filter(|x| x % 2 == 0).take(3).collect::<Vec<_>>());

    let fib = Fibbonacci::new();
    println!("{:?}",fib.take(10).collect::<Vec<_>>());

    let r: Vec<i32> = (1..=4)
        .inspect(|x| print!("a{} ", x)) // a1
        .filter(|x| x % 2 == 0)
        .inspect(|x| print!("b{} ", x)) // a2
        .take(1)
        .collect();
    println!("{:?}", r); // [2]

    let v = [1, 2, 3];
    let it = v.iter().map(|x| x + 1);
    println!("{}", it.clone().sum::<i32>());
    println!("{}", it.count());
}
