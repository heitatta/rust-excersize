use std::thread;
use core::time::Duration;

// closure を受け取る関数の例
fn apply<F: Fn(i32)->i32>(f: F, x: i32) -> i32 {
    f(x)
}
fn apply_twice<F: FnMut()>(mut f: F) {
    f();
    f();
}
fn run_once<F: FnOnce()->String>(f: F) -> String {
    f()
}

fn main() {

    // closure の例
    let add1 = |x| x + 1;
    let add2 = |x,y| x + y;
    let hello = || println!("Hello, world!");

    hello();
    println!("{}", add1(5));
    println!("{}", add2(3.0,4.0));
    // println!("{}", add2(3,4)); // 上で f64 x f64 に固定されるので、エラー

    //
    // 環境をキャプチャする例
    //
    let name = String::from("Alice");
    // greet は name を読むだけなので、& で借りたことになっている
    let greet = || format!("Hello, {}!", name);
    println!("{}", greet());
    // name はまだ使える
    println!("{}", name);

    //
    // mut で環境をキャプチャする例
    //
    let mut count = 0;
    // count を &mut で借りるので、inc 自身も mut になる
    let mut inc = || { count += 1; count };
    inc();
    // mutabe で貸し出し中なので、immutable で貸し出せない
    // println!("count = {}", count);
    inc();
    // inc() を最後に使った後でなら、count を参照できる
    println!("count = {}", count);

    //
    // 所有権ごとキャプチャする例
    //
    let s = String::from("Hello");
    // move は s の所有権をクロージャに移すためのキーワード
    let consume = move |t| println!("{} {}", s, t);
    consume("world");
    consume("Rust");
    // println!("{}", s) // すでに move されているので、s は使えない

    // thread を起こすときは move しないとエラーになる
    let mut v = vec![1, 2, 3];
    // move がないとエラーになる：data の参照がスレッドより長生きするか保証できないため
    let th = thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        println!("vector sum: {:?}", v.iter().sum::<i32>());
        v.push(10); // v は move されているので、push できる
    });

    // closureを呼び出す関数を呼び出す。
    println!("{}", apply(|x| x+1, 5));
    println!("{}", apply(i32::abs, -5));
    let mut count = 1;
    println!("count: {}", count);
    apply_twice(|| count += 1 );
    println!("count: {}", count);
    let s = String::from("owned");
    println!("{}", run_once(move || s));

    // 環境を取り込まない closure の型は fn (取り込む場合は Fn トレイトを実装した固有型になり、Box(dyn Fn) などで扱う必要がある)
    let v: Vec<fn(i32)->i32> = vec![
        |x| x + 1,
        |x| x * 2,
        |x| x * x,
    ];
    for f in &v {
        println!("{:?} {}", f, f(5));
    }
    fn compose(fs: Vec<fn(i32)->i32>) -> impl Fn(i32)->i32 {
        move |x| fs.iter().fold(x, |acc, f| f(acc))
    }
    println!("compose(0)={}", compose(v)(0));
    println!("fold sample: {}", [1,2,3,4,5].iter().fold(0, |x: i32, y| x + y + 1));

    // クロージャを返す関数
    fn addr(n: i32) -> impl Fn(i32) -> i32 { move |x| x + n }
    fn counter() -> impl FnMut() -> u32 {
        let mut count = 0;
        move || {
            count += 1; count
        }
    }
    println!("addr(5)(10) = {}", addr(5)(10));
    let mut c = counter();
    println!("c() = {}", c());
    println!("c() = {}", c());
    // 上で作ったスレッドをジョイン
    println!("{:?}", th.join());
}
