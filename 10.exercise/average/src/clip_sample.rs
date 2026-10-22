fn average(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in 0..v.len() {
        sum += v[i];
    }
    return sum as f64 / v.len() as f64;
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{}", average(&v));

    let s = String::from("abc");
    if s.len() == 0 { println!("empty"); }

    let x: Option<i32> = Some(1);
    match x {
        Some(n) => println!("{}", n),
        _ => (),
    }

    let t = true;
    if t == true { println!("yes"); }
}
