use config::{Config, ConfigError};

use std::fs;
use clap::Parser;
use std::process::ExitCode;

#[derive(Parser,Debug)]
#[command(version, about)]
struct Args {
    file: String,
}

/// 引数に与えられたファイルを config ファイルとして読み込む。
//
// 与えられたファイルをパースして、 成功か不成功かを exit codeで示す。
fn main() -> ExitCode {
    match read_config(&Args::parse().file) {
        Ok(config) => {
            println!("{:?}", config);
            ExitCode::SUCCESS
        },
        Err(e) => {
            eprintln!("Error reading config: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn read_config(filename: &str) -> Result<Config, ConfigError> {
    let text = fs::read_to_string(filename)?;
    let config = Config::parse(&text)?;
    Ok(config)
}
