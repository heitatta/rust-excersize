use std::collections::HashMap;

fn const_map(s: &str) -> HashMap<&str, &str> {
    let mut m = HashMap::new();
    for pair in s.split(", ") {
        let mut kv = pair.split("=");
        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
            m.insert(k, v);
        }
    }
    m
}

fn main() {
    let s = "name=taro, age=20, city=tokyo";
    let m = const_map(s);
    println!("{:?}", m);
}
