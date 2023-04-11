mod mapping;

use std::ops::Deref;

pub use mapping::Mapping;

pub struct Mappings(Vec<Mapping>);

impl Mappings {
    fn merge_overlapping(&mut self) {
        todo!("https://www.geeksforgeeks.org/merging-intervals/")
    }
}

impl Deref for Mappings {
    type Target = Vec<Mapping>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Mapping>> for Mappings {
    fn from(value: Vec<Mapping>) -> Self {
        let mut result = Self(value);
        result.merge_overlapping();
        result
    }
}

impl std::fmt::Display for Mappings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(value) = iter.next() {
            write!(f, "{value}")?;
            for value in iter {
                write!(f, ", {value}")?;
            }
        }
        Ok(())
    }
}
