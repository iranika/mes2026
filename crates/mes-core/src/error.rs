use std::fmt;

/// Fallible MeS conversion / config error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MesError {
    message: String,
    line: Option<usize>,
    column: Option<usize>,
}

impl MesError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    /// Error with a 1-based source line (and optional column).
    pub fn with_location(
        message: impl Into<String>,
        line: usize,
        column: Option<usize>,
    ) -> Self {
        Self {
            message: message.into(),
            line: Some(line),
            column,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn line(&self) -> Option<usize> {
        self.line
    }

    pub fn column(&self) -> Option<usize> {
        self.column
    }
}

impl fmt::Display for MesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.column) {
            (Some(line), Some(column)) => {
                write!(f, "{} (line {}, column {})", self.message, line, column)
            }
            (Some(line), None) => write!(f, "{} (line {})", self.message, line),
            _ => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for MesError {}

impl From<serde_json::Error> for MesError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(format!("json error: {value}"))
    }
}

impl From<regex::Error> for MesError {
    fn from(value: regex::Error) -> Self {
        Self::new(format!("regex error: {value}"))
    }
}

impl From<std::io::Error> for MesError {
    fn from(value: std::io::Error) -> Self {
        Self::new(format!("io error: {value}"))
    }
}

pub type MesResult<T> = Result<T, MesError>;
