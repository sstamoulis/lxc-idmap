use std::{fmt::Display, num::ParseIntError};

use super::NewMappingRangeError;

#[derive(Debug)]
pub struct ParseMappingRangeError {
    pub message: String,
}

impl Display for ParseMappingRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseMappingRangeError {}

impl From<ParseIntError> for ParseMappingRangeError {
    fn from(value: ParseIntError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<String> for ParseMappingRangeError {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}

impl From<NewMappingRangeError> for ParseMappingRangeError {
    fn from(value: NewMappingRangeError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
