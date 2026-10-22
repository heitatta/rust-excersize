fn main() {
    let mut s = String::from("Hello");
    let r1 = &s; // 不可変参照
    let r2 = &s; // 不可変参照
    println!("r1: {}, r2: {}", r1, r2);
    let r3 = &mut s; // 可変参照
    r3.push_str(", world!");
    println!("r3: {}", r3);
    // println!("r1: {}, r2: {}", r1, r2); // これはコンパイルエラーになります。なぜなら、r1とr2はすでにsの不変参照を持っているため、可変参照r3を作成することはできません。

    let s = String::from("a");
    let _t = s; // 所有権がtに移動
    // let u = s; // これはコンパイルエラーになります。なぜなら、sの所有権はすでにtに移動しているため、sを使用することはできません。

    let mut v = vec![1,2,3];
    let _first = &v[0]; // 不可変参照
    v.push(4); // 可変参照
    //println!("first: {}", _first); // これはコンパイルエラーになります。なぜなら、vの可変参照が作成された後に、vの不変参照firstを使用することはできないためです。
}

fn _f(s: &String) {
    // s.push('x') // これはコンパイルエラーになります。なぜなら、sは不変参照であり、可変参照ではないため、sを変更することはできません。
    println!("s: {}", s);
}
