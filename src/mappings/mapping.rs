mod mapping_range;
mod parse_mapping_error;

use parse_mapping_error::ParseMappingError;
use std::str::FromStr;

use self::mapping_range::MappingRange;

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    pub host_range: MappingRange,
    pub container_range: MappingRange,
}

impl FromStr for Mapping {
    type Err = ParseMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<&str>>().as_slice() {
            [host_range, container_range] => Ok(Mapping {
                host_range: host_range.parse()?,
                container_range: container_range.parse()?,
            }),
            [range] => {
                let range = range.parse::<MappingRange>()?;
                Ok(Mapping {
                    host_range: range.clone(),
                    container_range: range,
                })
            }
            _ => Err(format!("Mapping `{s}` is invalid").into()),
        }
    }
}

impl std::fmt::Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.host_range.start)?;
        if self.host_range.len() > 1 {
            write!(f, "-{}", self.host_range.end - 1)?;
        }
        write!(f, ":{}", self.container_range.start)?;
        if self.container_range.len() > 1 {
            write!(f, "-{}", self.container_range.end - 1)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapping_range(start: u32, end: u32) -> MappingRange {
        MappingRange::new(start, end).unwrap()
    }

    fn mapping_range_single(id: u32) -> MappingRange {
        MappingRange::new_single(id).unwrap()
    }

    #[test]
    fn parse_ranged() {
        assert_eq!(
            Mapping::from_str("1000-1005:1100-1105").unwrap(),
            Mapping {
                host_range: mapping_range(1000, 1005),
                container_range: mapping_range(1100, 1105),
            }
        )
    }

    #[test]
    fn parse_single() {
        assert_eq!(
            Mapping::from_str("1005:1010").unwrap(),
            Mapping {
                host_range: mapping_range_single(1005),
                container_range: mapping_range_single(1010),
            }
        )
    }

    #[test]
    fn parse_ranged_common() {
        assert_eq!(
            Mapping::from_str("1000-1005").unwrap(),
            Mapping {
                host_range: mapping_range(1000, 1005),
                container_range: mapping_range(1000, 1005),
            }
        )
    }

    #[test]
    fn parse_single_common() {
        assert_eq!(
            Mapping::from_str("1005").unwrap(),
            Mapping {
                host_range: mapping_range_single(1005),
                container_range: mapping_range_single(1005),
            }
        )
    }
}
