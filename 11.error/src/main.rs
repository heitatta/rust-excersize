use std::error::Error;
use std::fs;

mod readerror;
use readerror::ReadError;

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn read_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;  // could return io::Error
    let n = text.trim().parse::<i32>()?;   // could return ParseError
    if n < 0 {
        return Err("No Negative number accepted".into());
    }
    Ok(n)
}

fn read_number2(path: &str) -> Result<i32, ReadError> {
    let text = fs::read_to_string(path)?;  // could return io::Error
    let n = text.trim().parse::<i32>()?;   // could return ParseError
    if n < 0 {
        return Err(ReadError::Negative(n))
    }
    Ok(n)
}

// Option -> Error に変換する (ok_or)
fn first_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&n| n % 2 == 0).copied()
}

fn double_first_even(v: &[i32]) -> Result<i32, String> {
    let n = first_even(v).ok_or("偶数がありません".into());
    match n {
        Ok(n) => Ok(n * 2),
        Err(e) => Err(e),
    }
}

// Error -> Option に変換する (ok)
fn parse_int(s: &str) -> Option<i32> {
    // Error 情報は捨てられて Some() または None が返る
    s.parse::<i32>().ok()
}

// 関数の中で Option の ? と Result の ? が混ざる例。関数型に合わせる必要がある。
fn tripple_first_even(v: &[i32]) -> Option<i32> {
    let n = first_even(v)?; // Option の ? はそのまま使える
    let d = double_first_even(v).ok()?; // Result の ? は ok() で Option に変換してから使う
    Some(n + d)
}

fn main() {
    println!("10/2={}", divide(10,2).unwrap());
    println!("10/0={:?}", divide(10,0));
    println!("text: {:?}", read_number("number.txt"));
    println!("text: {:?}", read_number("no_such_file"));
    println!("text: {:?}", read_number("negative.txt"));
    println!("text: {:?}", read_number2("number.txt"));
    println!("text: {:?}", read_number2("no_such_file"));
    println!("text: {:?}", read_number2("negative.txt"));

    println!("double_first_even(&[1,2,3]) = {:?}",
             double_first_even(&[1,2,3]));
    println!("double_first_even(&[1,3,5]) = {:?}",
             double_first_even(&[1,3,5]));
    println!("parse_int(\"3.2\") = {:?}", parse_int("3.2"));
    println!("parse_int(\"3.2\") = {:?}", parse_int("3.2"));
    println!("tripple_first_even(&[1,2,3]) = {:?}",
             tripple_first_even(&[1,2,3]));

}
