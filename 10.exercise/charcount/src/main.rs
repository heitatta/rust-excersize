use std::collections::HashMap;

fn char_count(s: &str) -> HashMap<char, usize> {
    let mut m = HashMap::new();
    s.chars().for_each(|c| *m.entry(c).or_insert(0) += 1);
    m
}

fn main() {
    let s = "hello world";
    let m = char_count(s);
    println!("{:?}", m);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_count() {
        let m = char_count("hello world");
        assert_eq!(m.get(&'h'), Some(&1));
        assert_eq!(m.get(&'l'), Some(&3));
        assert_eq!(m.get(&'a'), None);
    }

    #[test]
    fn test_null_str() {
        let s = "";
        let m = char_count(s);
        assert_eq!(m.len(), 0);
        assert_eq!(m.get(&'h'), None);
    }
    #[test]
    fn test_long_str() {
        let s = "loooooooooooooooooooooooooooooooooooooooooooooooooong";
        let m = char_count(&s);
        assert_eq!(m.get(&'o'), Some(&50));
    }

}
