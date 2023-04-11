use std::fmt::Display;

#[derive(Debug)]
pub enum NewMappingRangeError {
    RootId,
    EndLessThanStart { start: u32, end: u32 },
}

impl Display for NewMappingRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewMappingRangeError::RootId => write!(f, "Cannot map root id `0`"),
            NewMappingRangeError::EndLessThanStart { start, end } => write!(
                f,
                "Mapping range end `{end}` cannot be less than it's start `{start}`"
            ),
        }
    }
}

impl std::error::Error for NewMappingRangeError {}
