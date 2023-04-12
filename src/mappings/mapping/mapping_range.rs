mod new_mapping_range_error;
mod parse_mapping_range_error;

use std::{
    ops::{Deref, DerefMut, Range},
    str::FromStr,
};

pub use self::{
    new_mapping_range_error::NewMappingRangeError,
    parse_mapping_range_error::ParseMappingRangeError,
};

/// A mapping range bounded inclusively below and above
#[derive(Debug, Clone, PartialEq)]
pub struct MappingRange(Range<u32>);

impl MappingRange {
    /// Returns a MappingRange with the given start and end (inclusive) bounds
    /// # Arguments
    /// * `start` - an id `> 0`
    /// * `end` - an id `>= start`
    pub fn new(start: u32, end: u32) -> Result<Self, NewMappingRangeError> {
        // end is inclusive
        let end = end + 1;
        if start == 0 {
            Err(NewMappingRangeError::RootId)
        } else if start > end {
            Err(NewMappingRangeError::EndLessThanStart { start, end })
        } else {
            Ok(MappingRange(Range { start, end }))
        }
    }

    /// Returns a [MappingRange] with both `start` and `end` set to the same `id`
    /// # Arguments
    /// * `id` - an id `> 0`
    pub fn new_single(id: u32) -> Result<Self, NewMappingRangeError> {
        Self::new(id, id)
    }
}

impl Deref for MappingRange {
    type Target = Range<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MappingRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for MappingRange {
    type Err = ParseMappingRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('-').collect::<Vec<&str>>().as_slice() {
            [start, end] => Ok(Self::new(start.parse()?, end.parse()?)?),
            [id] => Ok(Self::new_single(id.parse()?)?),
            _ => Err(format!("Id range `{s}` is invalid").into()),
        }
    }
}
