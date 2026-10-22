use std::collections::HashMap;

fn main() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("apple".to_string(), 3);
    m.insert("banana".to_string(), 5);

    println!("{:?}", m);
    println!("get apple: {:?}", m.get("apple"));  // Option が返る
    println!("get cherry: {:?}", m.get("cherry")); // 存在しない Key

    println!("apple: {}", m["apple"]);  // 存在しないキーならパニック
    println!("does contain banana?: {}", m.contains_key("banana"));

    m.remove("banana");      // Some(5)
    println!("{:?}", m);

    let key = "apple".to_string();
    m.insert(key, 7);
    // println!("{:?}", key); // key は所有権が移動しているので使えない
    println!("{:?}", m);

    // 無ければ入れ、あれば更新するイディオム
    let mut m = HashMap::new();
    m.insert("apple".to_string(), 3);
    println!("{:?}", m);

    *m.entry("apple".to_string()).or_insert(0) += 1;
    *m.entry("cherry".to_string()).or_insert(0) += 1;
    println!("{:?}", m);

    // iteration
    for k in m.keys() {
        println!("key = {}, value = {}", k, m[k]);
    }
}
