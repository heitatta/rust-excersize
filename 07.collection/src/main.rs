fn vec_basic() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3); v.push(1); v.push(2);
    println!("v = {:?}", v);
    println!("v.len() = {}", v.len());
    println!("v[0] = {}", v[0]);
    println!("v.get(1) = {:?}", v.get(1));
    println!("v.get(3) = {:?}", v.get(3));
    println!("v.pop() = {:?}", v.pop());
    println!("v.len() = {}", v.len());
    for i in v {
        println!("i = {}", i);
    }
    // println!("v = {:?}", v);  // v は消費されているのでエラー
}

fn vec_ops(){
    let mut v = vec![1,8,7,3,7,1,5];
    for x in &v {
        println!("x = {}", x);
    }
    for x in v.iter_mut() {
        *x *= 10;
    }
    v.sort(); println!("v = {:?}", v);
    v.dedup(); println!("v = {:?}", v);
    v.reverse(); println!("v = {:?}", v);
    v.insert(0,99); println!("v = {:?}", v);
    v.retain(|&x| x % 3 == 0); println!("v = {:?}", v);
}

fn vec_float() {
    let mut v: Vec<f64> = Vec::new();
    v.push(3.14); v.push(2.71); v.push(1.41);
    println!("v = {:?}", v);
    // v.sort(); println!("v = {:?}", v); // error
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("v = {:?}", v);
}

fn vec_itr() {
    let v = vec![1, 2, 3, 4, 5];
    println!("sum(): {}", v.iter().sum::<i32>());
    let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("mapped: {:?}", v);
    println!("mapped: {:?}", v2);
}

fn vec_ownership() {
    let mut v = vec![String::from("abc"), String::from("def")];
    let s = &mut v[0];
    *s = s.to_uppercase();
    println!("v = {:?}", v);
}

fn main() {
    vec_basic();
    vec_ops();
    vec_float();
    vec_itr();
    vec_ownership();
}
