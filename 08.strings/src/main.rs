use std::num::ParseIntError;

fn str_basic() {
    let a = String::new();
    let b = "hello".to_string();
    let mut c = String::from("hello");
    c.push(' ');
    c.push_str("world");
    println!("a = {}, b = {}, c = {}", a, b, c);
}

fn str_concat() {
    let a = String::from("hello");
    let b = String::from("world");

    println!("{}", format!("{} {}", a, b)); // 所有権は移転しない

    let c = a + " " + &b;  // a の所有権は c に移転する
    println!("c = {}", c);
    // println!("a = {}", a); // ここではもう a は使えない
    println!("b = {}", b);
}

fn str_ops() {
    let t = "  Hello, World  ";
    println!("t.trim() = {}", t.trim());
    println!("t.to_lowercase() = {}", t.to_lowercase());
    println!("t contains \"World\"?: {}", t.contains("World"));
    println!("starts with \"Hello\": {}", t.trim().starts_with("Hello"));
    println!("replace: {}", t.replace("World", "Rust!"));
    println!("t: {}", t); // t は変更されていない
}

fn str_convert() {
    let s = "123";
    let n: Result<i32,ParseIntError> = s.parse();
    println!("n = {:?}", n);
}

fn str_unicode() {
    let s = "こんにちは";
    println!("s の長さ = {}", s.len());
    println!("s の文字列 = {}", s.chars().count());
    s.chars().for_each(|c| println!("char = {}", c));
}

fn shout(s: &str) -> () {
    println!("{}!" , s.to_uppercase());
}

fn main() {
    str_basic();
    str_concat();
    str_ops();
    str_convert();
    str_unicode();

    let s = String::from("hello");
    shout(&s); // スライスで渡す
    shout("world"); // リテラルで渡す(リテラルは &str 型)
    // shout(s); // これはエラー(String は &str に自動変換されない)
}
