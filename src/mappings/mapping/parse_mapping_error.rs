use super::mapping_range::{NewMappingRangeError, ParseMappingRangeError};

#[derive(Debug)]
pub struct ParseMappingError {
    pub message: String,
}

impl From<String> for ParseMappingError {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}

impl From<std::num::ParseIntError> for ParseMappingError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<NewMappingRangeError> for ParseMappingError {
    fn from(value: NewMappingRangeError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<ParseMappingRangeError> for ParseMappingError {
    fn from(value: ParseMappingRangeError) -> Self {
        Self {
            message: value.message,
        }
    }
}

impl std::fmt::Display for ParseMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseMappingError {}
