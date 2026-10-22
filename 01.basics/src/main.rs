// 関数定義
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn classify(n: i32) -> &'static str {
    if n < 0 {
        "負"
    } else if n == 0 {
        "ゼロ"
    } else {
        "正"
    }
}

fn sum_to(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn main() {
    let result = add(5, 3);
    println!("The sum is: {}", result);

    for n in [-5, 0, 7] {
        println!("{} は{}です。", n, classify(n));
    }

    println!("1 から 10 までの合計は: {}", sum_to(10));

    let mut count: i32 = 3;
    while count > 0 {
        println!("カウントダウン: {}", count);
        count -= 1;
    }

    let mut i = 0;
    let result = loop {
        i += 1;
        if i*i > 50 {
            break i;
        }
    };
    println!("最初の平方が 50 を超える整数は: {}", result);
}
