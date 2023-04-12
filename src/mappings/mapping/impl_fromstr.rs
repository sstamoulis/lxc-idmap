use std::str::FromStr;

use super::{Mapping, ParseMappingError};

struct MappingRange {
    start: u32,
    count: Option<u32>,
}

impl FromStr for MappingRange {
    type Err = ParseMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('-').collect::<Vec<&str>>().as_slice() {
            [start, end] => {
                let start: u32 = start.parse()?;
                let end: u32 = end.parse()?;
                if end > start {
                    Ok(MappingRange {
                        start,
                        count: Some(end - start + 1),
                    })
                } else {
                    Err(format!(
                        "Id range's `{s}` start `{start}` \
                         is greater than or equal to its end `{end}`"
                    )
                    .into())
                }
            }
            [start] => Ok(MappingRange {
                start: start.parse()?,
                count: None,
            }),
            _ => Err(format!("Id range `{s}` is invalid").into()),
        }
    }
}

impl FromStr for Mapping {
    type Err = ParseMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<&str>>().as_slice() {
            [ct_range_text, host_range_text] => {
                let ct: MappingRange = ct_range_text.parse()?;
                let host: MappingRange = host_range_text.parse()?;
                let count = ct.count.unwrap_or(1);
                match host.count {
                    Some(host_count) if count != host_count => Err(format!(
                        "The count of ids in the container \
                         range `{ct_range_text}` is different \
                         from that in the host range `{host_range_text}`"
                    )
                    .into()),
                    _ => Ok(Mapping {
                        ct_start: ct.start,
                        host_start: host.start,
                        count,
                    }),
                }
            }
            [range] => {
                let range: MappingRange = range.parse()?;
                Ok(Mapping {
                    ct_start: range.start,
                    host_start: range.start,
                    count: range.count.unwrap_or(1),
                })
            }
            _ => Err(format!("Mapping `{s}` is invalid").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ranged() {
        assert_eq!(
            "1000-1005:1100".parse::<Mapping>().unwrap(),
            Mapping {
                ct_start: 1000,
                host_start: 1100,
                count: 6
            }
        )
    }

    #[test]
    fn parse_single() {
        assert_eq!(
            "1005:1010".parse::<Mapping>().unwrap(),
            Mapping {
                ct_start: 1005,
                host_start: 1010,
                count: 1
            }
        )
    }

    #[test]
    fn parse_ranged_common() {
        assert_eq!(
            "1000-1005".parse::<Mapping>().unwrap(),
            Mapping {
                ct_start: 1000,
                host_start: 1000,
                count: 6
            }
        )
    }

    #[test]
    fn parse_single_common() {
        assert_eq!(
            "1005".parse::<Mapping>().unwrap(),
            Mapping {
                ct_start: 1005,
                host_start: 1005,
                count: 1
            }
        )
    }
}
