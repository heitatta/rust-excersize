use std::f64::consts::PI;
use std::fmt::Display;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> String {
        String::from("図形")
    }
    fn describe(&self) -> String {
        format!("{}の周囲は {:.2}、面積は{:.2}です。", self.name(), self.perimeter(), self.area())
    }
}

#[derive(Default)]
struct Circle { r: f64 }
struct Rect { w: f64, h: f64 }

// Shape Trait を実装する例
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.r
    }
    fn name(&self) -> String {
        format!("Circle({})", self.r)
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle({})", self.r)
    }
}

// Shape Trait を実装する例
impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.w + self.h)
    }
    fn name(&self) -> String {
        format!("Rect({},{})", self.w, self.h)
    }
}

impl Display for Rect{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect({}, {})", self.w, self.h)
    }
}

// Trait を使ったジェネリック関数の例
fn print_area<T: Shape>(s: &T) { // print_area(s: &impl Shape) でも OK
    println!("{}", s.describe());
}

// 動的ディスパッチを使った関数の例
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    // total_area(shapes: &[T]) -> f64 where T: Shape  だと、
    // T は固定(Circle など)になるが、実行時ディスパッチがないので速い。
    // Box は異なるサイズの型をヒープに置くための入れ物
    shapes.iter().map(|s| s.area()).sum()
}

fn largest<T>(v: &[T]) -> Option<&T> where T: PartialOrd {
    let mut largest = &v[0];
    for item in v {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 { fn describe(&self) -> String { format!("整数: {}", self) } }
impl Describe for String { fn describe(&self) -> String { format!("文字列: {}", self) } }
impl Describe for str { fn describe(&self) -> String { format!("文字列: {}", self) } }
impl<T: Describe> Describe for Vec<T> {
    fn describe(&self) -> String {
        format!("[{}]", self.iter().map(|item| item.describe()).collect::<Vec<_>>().join(", "))
    }
}

impl<T: Describe + ?Sized> Describe for Box<T> {
    fn describe(&self) -> String {
        self.as_ref().describe()
    }
}

fn largest_by_ara(shape: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    let mut iter = shape.iter();
    let mut largest = iter.next()?.as_ref();
    for s in iter {
        if s.area() > largest.area() {
            largest = s.as_ref();
        }
    }
    Some(largest)
}

fn main() {
    let r = 3.0;
    let circle = Circle { r };
    let rectangle = Rect { w: 2.0, h: 3.0 };
    println!("{}", Circle::default().describe());
    println!("{}", circle.describe());
    print_area(&rectangle);
    println!("total: {:.2}", total_area(&[Box::new(circle), Box::new(rectangle)]));

    println!("largest: {:?}", largest(&[1, 2, 3, 4, 5]));
    println!("largest: {:?}", largest(&["B", "Z", "A", "p"]));
    println!("describe: {}", vec![1, 2].describe());
    let v: Vec<Box<dyn Describe>> = vec![Box::new(1), Box::new(String::from("a"))];
    println!("describe: {}", v.describe());
    let v = vec![Box::new(1), Box::new(2)];
    println!("describe: {}", v.describe());

    let mut v: Vec<Box<dyn Shape>> = vec![Box::new(Circle { r: 3.0 }),
                                          Box::new(Circle { r: 5.0 }),
                                          Box::new(Rect { w: 2.0, h: 3.0 }),
                                          Box::new(Rect { w: 3.0, h: 8.0 }),
                                          Box::new(Rect { w: 4.0, h: 1.0 }),
    ];
    v.sort_by(|s1,s2| s1.area().partial_cmp(&s2.area()).unwrap());
    println!("sorted by area: {}", v.iter().map(|s| s.name()).collect::<Vec<_>>().join(", "));

    println!("largest area: {}", largest_by_ara(&v).unwrap().name());
}
