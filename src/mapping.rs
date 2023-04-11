use std::{ops::Range, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    pub host_range: Range<u32>,
    pub container_range: Range<u32>,
    pub mapping_type: MappingType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MappingType {
    Uid,
    Gid,
    Both,
}

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

impl Mapping {
    pub fn from_str_with_type(
        s: &str,
        mapping_type: MappingType,
    ) -> Result<Self, ParseMappingError> {
        fn parse_range(s: &str) -> Result<Range<u32>, ParseMappingError> {
            match s.split('-').collect::<Vec<&str>>().as_slice() {
                [start, end] => Ok(Range {
                    start: start.parse()?,
                    end: end.parse::<u32>()? + 1,
                }),
                [id] => {
                    let id = id.parse()?;
                    Ok(Range { start: id, end: id })
                }
                _ => Err(format!("Id range `{s}` is invalid").into()),
            }
        }

        match s.split(':').collect::<Vec<&str>>().as_slice() {
            [host_range, container_range] => Ok(Mapping {
                host_range: parse_range(host_range)?,
                container_range: parse_range(container_range)?,
                mapping_type,
            }),
            [range] => {
                let range = parse_range(range)?;
                Ok(Mapping {
                    host_range: range.clone(),
                    container_range: range,
                    mapping_type,
                })
            }
            _ => Err(format!("Mapping `{s}` is invalid").into()),
        }
    }
}

impl FromStr for Mapping {
    type Err = ParseMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_type(s, MappingType::Both)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ranged_mapping() {
        assert_eq!(
            Mapping::from_str("1000-1005:1100-1105").unwrap(),
            Mapping {
                host_range: 1000..1005 + 1,
                container_range: 1100..1105 + 1,
                mapping_type: MappingType::Both
            }
        )
    }

    #[test]
    fn parse_single_mapping() {
        assert_eq!(
            Mapping::from_str("1005:1010").unwrap(),
            Mapping {
                host_range: 1005..1005,
                container_range: 1010..1010,
                mapping_type: MappingType::Both
            }
        )
    }

    #[test]
    fn parse_ranged_common_mapping() {
        assert_eq!(
            Mapping::from_str("1000-1005").unwrap(),
            Mapping {
                host_range: 1000..1005 + 1,
                container_range: 1000..1005 + 1,
                mapping_type: MappingType::Both
            }
        )
    }

    #[test]
    fn parse_single_common_mapping() {
        assert_eq!(
            Mapping::from_str("1005").unwrap(),
            Mapping {
                host_range: 1005..1005,
                container_range: 1005..1005,
                mapping_type: MappingType::Both
            }
        )
    }
}
