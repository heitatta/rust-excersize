use std::fmt;

#[derive(Debug)]
/// [`ConfigError`] は、設定ファイルのパース中に発生するエラーを表す列挙型。
pub enum ConfigError {

    /// 不正な行があった場合のエラー。
    InvalidLine {
        /// 行番号 (1 から始まる)
        line_num: usize,
        /// 行の内容
        line: String
    },

    /// キーが空の行があった場合のエラー。行番号と行の内容を保持する。
    EmptyKey {
        /// 行番号 (1 から始まる)
        line_num: usize,
        /// 行の内容
        line: String
    },

    /// 重複するキーがあった場合のエラー。行番号と行の内容を保持する。
    DuplicateKey {
        /// 行番号 (1 から始まる)
        line_num: usize,
        /// 行の内容
        line: String
    },

    /// IO エラーが発生した場合のエラー。元の IO エラーを保持する。
    Io(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidLine{line_num, line} => write!(f, "Invalid line ({}): {}", line_num, line),
            ConfigError::EmptyKey{line_num, line} => write!(f, "Empty Key ({}): {}", line_num, line),
            ConfigError::DuplicateKey{line_num, line} => write!(f, "Duplicate Key ({}): {}", line_num, line),
            ConfigError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::InvalidLine{..} => None,
            ConfigError::EmptyKey{..} => None,
            ConfigError::DuplicateKey{..} => None,
            ConfigError::Io(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}
