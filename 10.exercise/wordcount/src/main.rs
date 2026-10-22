use std::fs::read_to_string;
use std::collections::HashMap;
use std::process::ExitCode;
use clap::Parser;
use std::error::Error;

#[derive(Parser,Debug)]
#[command(version, about)]
struct Args {
    file: String,

    // 先頭から n 行を対象にする
    #[arg(short='n', long, value_name = "N")]
    max_lines: Option<usize>,
}

fn first_lines(text: &str, n: usize) -> &str {
    let end: usize = text.split_inclusive('\n').take(n).map(str::len).sum();
    &text[..end]
}

fn word_count(text: &str, lines: Option<usize>) -> HashMap<&str, usize> {
    let mut m = HashMap::new();
    let words: &str = match lines {
        Some(n) => first_lines(text, n),
        None => text,
    };
    for word in words.split_whitespace() {
        *m.entry(word).or_insert(0) += 1;
    }
    m
}

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let path = &args.file;
    let text = read_to_string(path)?;
    let m = word_count(&text, args.max_lines);

    // 出現回数で降順にソートして表示
    let mut v = m.iter().collect::<Vec<_>>();
    v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (word, count) in v {
        println!("{}: {}", word, count);
    };
    Ok(())
}

fn main() -> ExitCode {
    match run(&Args::parse()) {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}


#[cfg(test)]
#[test]
fn counts_words_with_line_limit() {
    let m = word_count("a b a\nc a\n", Some(1));
    assert_eq!(m.get("a"), Some(&2));
    assert_eq!(m.get("c"), None);
}

#[test]
fn test_first_lines() {
    let text = "line1\nline2\nline3\nline4\n";
    assert_eq!(first_lines(text, 2), "line1\nline2\n");
    assert_eq!(first_lines(text, 0), "");
    assert_eq!(first_lines(text, usize::MAX), text);
}

#[test]
fn test_first_lines_dos() {
    let text = "line1\r\nline2\r\nline3\r\nline4\r\n";
    assert_eq!(first_lines(text, 2), "line1\r\nline2\r\n");
    assert_eq!(first_lines(text, 0), "");
    assert_eq!(first_lines(text, usize::MAX), text);
}

#[test]
fn non_existing_file() {
    let args = Args { file: "no_such_file.txt".to_string(), max_lines: None };
    assert!(run(&args).is_err());
}
