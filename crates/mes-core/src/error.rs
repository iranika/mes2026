use std::fmt;

/// Fallible MeS conversion / config error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MesError {
    message: String,
}

impl MesError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for MesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
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

pub type MesResult<T> = Result<T, MesError>;
