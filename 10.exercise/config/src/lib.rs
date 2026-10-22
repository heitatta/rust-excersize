//! 設定ファイル（`key = value` 形式）を読み込むライブラリ。
//!
//! # Examples
//!
//! ```
//! use config::{Config,ConfigError};
//!
//! let config: Config = "name = taro".parse().unwrap();
//! assert_eq!(config.get("name"), Some("taro"));
//! ```
//!
//! # Errors
//! - `=` の無い行があるとき: [`ConfigError::InvalidLine`]
//! - キーが空の行があるとき: [`ConfigError::EmptyKey`]
//! - キーが重複していたとき: [`ConfigError::DuplicateKey`]
//!
//! ```
//! # use config::{Config,ConfigError};
//! let err = "x".parse::<Config>().unwrap_err();
//! assert!(matches!(err, ConfigError::InvalidLine { line_num: 1, .. }));
//! ```

mod error;
pub use error::ConfigError;

use std::collections::HashMap;
use std::str::FromStr;

/// [`Config`] は、`key=value` の形式の設定ファイルを読み込むための構造体。
///
/// [`FromStr`] を実装しているので、[`str::parse`] で作れる。
/// 失敗すると [`ConfigError::InvalidLine`] などを返す。
#[derive(Debug)]
#[warn(missing_docs)]
pub struct Config {
    map: HashMap<String,String>,
}

/// [`Config`] の実装。
impl Config {
    /// [`Config`] を作る。
    pub fn new() -> Self {
        Config { map: HashMap::new() }
    }

    /// [`Config`] を文字列から作る。
    pub fn parse(text: &str) -> Result<Config, ConfigError> {
        let mut map = HashMap::new();
        for (i, line) in text.lines().enumerate() {
            let line = line.trim();
            let line_num = i + 1;
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line.split_once('=')
                .ok_or(ConfigError::InvalidLine{line_num,
                                                line: line.to_string()})?;
            // "=value"の行はエラー
            let key = key.trim();
            if key.is_empty() {
                return Err(ConfigError::EmptyKey{line_num, line: line.to_string()});
            }
            // 重複するキーはエラー
            if map.contains_key(key) {
                return Err(ConfigError::DuplicateKey{line_num, line: line.to_string()});
            }
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
        Ok(Config { map })
    }

    /// [`Config`] から値を取得する。
    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(String::as_str)
    }
}

/// [`Config`] のデフォルト値を作るためのトレイト実装。
impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

/// [`Config`] を文字列から作るためのトレイト実装。
impl FromStr for Config {
    /// [`ConfigError`] を返す。
    type Err = ConfigError;

    /// [`str::parse`] で [`Config`] を作る。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config: Config = Config::parse(s)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let text = r#"
          # This is a comment
          key1 = value1
          key2 = value2

          # Another comment
          url=http://x/?a=b
        "#;
        let config = Config::parse(text).unwrap();
        assert_eq!(config.get("key1"), Some("value1"));
        assert_eq!(config.get("key2"), Some("value2"));
        assert_eq!(config.get("url"), Some("http://x/?a=b"));
    }

    #[test]
    fn with_parse() {
        let text = r#"
          # This is a comment
          key1 = value1
          key2 = value2

          # Another comment
          url=http://x/?a=b
        "#;
        let config = text.parse::<Config>();
        match config {
            Ok(config) => {
                assert_eq!(config.get("key1"), Some("value1"));
                assert_eq!(config.get("key2"), Some("value2"));
                assert_eq!(config.get("url"), Some("http://x/?a=b"));
            }
            Err(e) => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn wih_from_str() {
        let c = Config::from_str("a=1").unwrap();
        assert_eq!(c.get("a"), Some("1"));
    }

    #[test]
    fn error_by_parse() {
        let e = "a=1\nbroken\n".parse::<Config>().unwrap_err();
        assert!(matches!(e, ConfigError::InvalidLine { line_num: 2, .. }));
    }

    #[test]
    fn question_mark_works_in_caller() {
        fn load(s: &str) -> Result<usize, ConfigError> {
            let c: Config = s.parse()?;      // Err = ConfigError なので ? がそのまま使える
            Ok(c.map.len())
        }
        assert_eq!(load("a=1\nb=2").unwrap(), 2);
        assert!(load("oops").is_err());
    }

    #[test]
    fn test_parse_error_no_equal() {
        match Config::parse("a=1\nbroken\n") {
            Err(ConfigError::InvalidLine { line_num, line }) => {
                assert_eq!(line_num, 2);
                assert_eq!(line, "broken");
            }
            other => panic!("unexpected: {:?}", other),
        }
    }

    #[test]
    fn test_parse_error_empty_key() {
        let text = r#"
            =value
        "#;
        let result = Config::parse(text);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_dup_key() {
        let text = r#"
            key=value1
            key=value2
        "#;
        let result = Config::parse(text);
        assert!(result.is_err());
    }
}
