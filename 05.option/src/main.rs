use std::env;
use std::process::ExitCode;

fn parse(s: &str) -> Option<i32> { s.parse().ok() }
fn half(n: i32) -> Option<i32> { if n % 2 == 0 { Some(n / 2) } else { None } }

fn f1(s: &str) -> Option<i32> {
    parse(s).and_then(half).map(|n| n + 1)
}

fn f2(s: &str) -> Option<i32> {
    let n = parse(s)?;
    let h = half(n)?;
    Some(h + 1)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <number>", args[0]);
        return ExitCode::FAILURE;
    }
    let input: &str = &args[1];
    let result1 = f1(input);
    let result2 = f2(input);
    match result1 {
        Some(value) => println!("Half+1 of {} is {}", input, value),
        None => println!("Could not compute half of {}", input)
    }
    match result2 {
        Some(value) => println!("Half+1 of {} is {}", input, value),
        None => println!("Could not compute half of {}", input)
    }
    ExitCode::SUCCESS
}
