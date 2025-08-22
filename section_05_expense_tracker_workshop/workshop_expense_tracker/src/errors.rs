use std::fmt;

#[derive(Debug)]
pub enum ExpenseError {
    IoError(std::io::Error),
    CsvError(csv::Error),
    ParseError(String),
    DateParseError(chrono::format::ParseError),
    InvalidEntryType(String),
}

impl fmt::Display for ExpenseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpenseError::IoError(err) => write!(f, "IO Error: {err}"),
            ExpenseError::CsvError(err) => write!(f, "CSV Error: {err}"),
            ExpenseError::ParseError(msg) => write!(f, "Parse Error: {msg}"),
            ExpenseError::DateParseError(err) => write!(f, "Date Parse Error: {err}"),
            ExpenseError::InvalidEntryType(entry_type) => {
                write!(f, "Invalid entry type: {entry_type}")
            }
        }
    }
}

impl std::error::Error for ExpenseError {}

impl From<std::io::Error> for ExpenseError {
    fn from(err: std::io::Error) -> Self {
        ExpenseError::IoError(err)
    }
}

impl From<csv::Error> for ExpenseError {
    fn from(err: csv::Error) -> Self {
        ExpenseError::CsvError(err)
    }
}

impl From<chrono::format::ParseError> for ExpenseError {
    fn from(err: chrono::format::ParseError) -> Self {
        ExpenseError::DateParseError(err)
    }
}

impl From<std::num::ParseFloatError> for ExpenseError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ExpenseError::ParseError(format!("ไม่สามารถแปลงเป็นตัวเลขได้: {err}"))
    }
}