fn average(xs: &[i32]) -> Option<f64> {
    let sum: i32 = xs.iter().sum();
    let count = xs.len();
    if count == 0 {
        None
    } else {
        Some(f64::from(sum) / count as f64)
    }
}

fn main() {
    println!("{:?}", average(&[10, 20, 30, 40]));
    println!("{:?}", average(&[3, 4, 6]));

    let av = average(&[]);
    println!("{:?}", av);
    match av {
        Some(n) => println!("average = {}", n),
        None => println!("empty!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_return_none() {
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn single_element() {
        assert_eq!(average(&[42]), Some(42.0));
    }

    #[test]
    fn multiple_elements() {
        assert_eq!(average(&[1, 2, 3]), Some(2.0));
    }

    #[test]
    fn negative_elements() {
        assert_eq!(average(&[-1, -2, -3]), Some(-2.0));
    }

    #[test]
    fn frational_average() {
        let r = average(&[1, 2, 4]).unwrap();
        assert!((r - 7.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn overflow() {
        average(&[i32::MAX, 1]);
    }
}
