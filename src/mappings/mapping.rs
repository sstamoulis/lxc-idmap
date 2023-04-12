mod impl_fromstr;
mod parse_mapping_error;

pub use parse_mapping_error::ParseMappingError;

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    pub ct_start: u32,
    pub host_start: u32,
    pub count: u32,
}

impl Mapping {
    pub fn ct_end(&self) -> u32 {
        self.ct_start + self.count
    }
}

impl std::fmt::Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ct_start)?;
        if self.count > 1 {
            write!(f, "-{}", self.ct_end() - 1)?;
        }
        if self.ct_start != self.host_start {
            write!(f, ":{}", self.host_start)?;
        }
        Ok(())
    }
}
