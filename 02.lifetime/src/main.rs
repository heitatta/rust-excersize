fn error1() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    //println!("{}", s1);
    println!("{}", s2);
} // This will cause a compile-time error because s1 has been moved to s2 and is no longer valid. To fix this, you can either clone s1 or use a reference.


fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
} // This function returns a reference to the longer of two string slices. The lifetime 'a ensures that the returned reference is valid as long as both input references are valid.

fn longest2(x: &str, y: &str) -> String {
    if x.len() >= y.len() { x.to_string() } else { y.to_string() }
} // This function returns a new String that is the longer of two string slices. It does not return a reference, so it does not have lifetime issues.

fn pick_first<'a>(x: &'a str, _y: &str) -> &'a str { x }

struct Excerpt<'a> {
    part: &'a str,
} // This struct holds a reference to a string slice with a specific lifetime 'a.

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 { 3 }
    fn first_word(&self) -> &str {
        self.part.split_whitespace().next().unwrap()
    }
} // This implementation block defines methods for the Excerpt struct. The method announce_and_return_part takes a reference to a string slice and returns a reference to the part field of the struct.


fn main() {
    error1();
    let a = String::from("hello");
    let r1;
    let r2;
    {
        let b = String::from("hi");
        r1 = longest1(&a, &b);
        r2 = longest2(&a, &b);
        println!("{}", r1); // a,b の両方が活きているスコープ内では r も参照できる
    } // b がスコープ外になったため、r はダングリング参照になり、コンパイル時エラーが発生します。これを修正するには、両方の参照が十分に長く存続するようにするか、ローカル変数への参照を返さないようにします。
    // println!("{}", r); // r はダングリング参照であるため、この行はコンパイル時エラーを引き起こします。
    println!("{}", r2); // r2 は String を返すため、スコープ外でも有効です。

    let a = String::from("long lived");
    let r;
    {
        let b = String::from("short");
        r = pick_first(&a, &b)
    }
    println!("r: {}", r); // r は a の参照を返すため、a が有効な限り r も有効です。

    let e;
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap();
    e = Excerpt { part: &first };
    println!("Excerpt {} (level: {})", e.first_word(), e.level());
}
