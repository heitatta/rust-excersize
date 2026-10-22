use std::fmt;
mod command;
use command::Command;

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Point,
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rect { w, h } => w * h,
            Shape::Point => 0.0,
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Circle(r) => write!(f, "円(半径{})", r),
            Shape::Rect { w, h } => write!(f, "長方形({}x{})", w, h),
            Shape::Point => write!(f, "点"),
        }
    }
}

fn sum(xs: &[i32]) -> i32 {
    let mut total = 0;
    for x in xs {
        total += x;
    }
    total
}

fn max_of(xs: &[i32]) -> Option<i32> {
    if xs.is_empty() {
        return None;
    }
    let mut max = xs[0];
    for &x in xs.iter().skip(1) {
        if x > max {
            max = x;
        }
    }
    Some(max)
}

fn first_word(s: &str) -> &str {
    let w = s.find(" ");
    match w {
        Some(i) => &s[0..i],
        None => s,
    }
}

fn main() {
    let shape = vec![Shape::Circle(2.0),
                     Shape::Rect { w: 3.0, h: 4.0},
                     Shape::Point];
    for s in &shape {
        println!("{}({:?}) area = {}", s, s, s.area());
        if let Shape::Circle(r) = s {
            println!("     これは半径 {} の円です。", r);
        }
    }

   match "12.5".parse::<i32>() {
        Ok(n) => println!("12 を整数に変換しました: {}", n),
        Err(e) => println!("12 を整数に変換できませんでした: {}", e),
    }

    let arr = [1, 2, 3, 4, 5];
    let v = vec![10, 20, 30];
    println!("sum of arr: {}", sum(&arr));
    println!("sum of vec: {}", sum(&v));
    println!("sum of arr slice: {}", sum(&arr[1..4]));

    let s = String::from("こんにちは");
    let cs: Vec<char> = s.chars().collect();
    println!("s[0..6] = {}", &s[0..6]);  // OK
    println!("{:?}", cs[1]);  // OK
    //println!("s[0..4] = {}", &s[0..4]);  // panic!

    let commands = vec![
        Command::Move { x: 10, y: 20 },
        Command::Say(String::from("Hello")),
        Command::Quit,
    ];
    for c in commands {
        c.run();
    }

    println!("max: {}", match max_of(&[1, 2, 3, 4, 5]) {
        Some(n) => n.to_string(),
        None => String::from("None"),
    });
    println!("{}", first_word("Hello, world!"));
    println!("{}", first_word("こんにちは 世界!"));
}
