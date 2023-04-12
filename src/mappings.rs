mod mapping;

use std::ops::Deref;

pub use mapping::Mapping;

#[derive(Debug, PartialEq)]
pub struct Mappings(Vec<Mapping>);

impl Mappings {
    fn merge_overlapping(self) -> Self {
        let mut iter = {
            let mut v = self.0;
            v.sort_unstable_by_key(|m| m.ct_start);
            v.into_iter()
        };
        let mut stack = Vec::new();
        if let Some(mut last_m) = iter.next() {
            for m in iter {
                if
                // is ct_range overlapping?
                // current mapping's ct_start is always >= to the previous one's since the vector is sorted
                m.ct_start <= last_m.ct_start + last_m.count &&
                // is host_range overlapping?
                m.host_start >= last_m.host_start &&
                m.host_start <= last_m.host_start + last_m.count
                {
                    // mappings are overlapping
                    let ct_end = (last_m.ct_start + last_m.count).max(m.ct_start + m.count);
                    last_m.count = ct_end - last_m.ct_start;
                } else {
                    // mappings are not overlapping
                    stack.push(last_m);
                    last_m = m;
                }
            }
            stack.push(last_m);
        }
        Mappings(stack)
    }

    /// Adds missing mappings to both ends and between provided mappings
    /// so that mappings are provided for the container's IDs from 0 up to and including 65535.
    ///
    /// For more information reference https://linuxcontainers.org/lxd/docs/latest/userns-idmap/
    pub fn with_missing(self) -> Self {
        let mut host_start = 1_000_000; // TODO: get this value from container or global lxc config
        let mut iter = self.0.into_iter();
        let mut mappings = Vec::new();
        if let Some(mut last_m) = iter.next() {
            if last_m.ct_start > 0 {
                mappings.push(Mapping {
                    ct_start: 0,
                    host_start,
                    count: last_m.ct_start,
                });
                host_start += last_m.ct_start;
            }
            mappings.push(last_m.clone());
            host_start += last_m.count;
            for m in iter {
                if m.ct_start > last_m.ct_end() {
                    let count = m.ct_start - last_m.ct_end();
                    mappings.push(Mapping {
                        ct_start: last_m.ct_end(),
                        host_start,
                        count,
                    });
                    host_start += count;
                }
                last_m = m.clone();
                mappings.push(m);
                host_start += last_m.count;
            }
            const CT_END: u32 = 65_535;
            if last_m.ct_end() < CT_END {
                mappings.push(Mapping {
                    ct_start: last_m.ct_end(),
                    host_start,
                    count: CT_END - last_m.ct_end() + 1,
                })
            }
        }
        Self(mappings)
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
        let result = Self(value);
        result.merge_overlapping()
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

#[cfg(test)]
mod tests {
    use super::*;

    impl From<&[&str]> for Mappings {
        fn from(value: &[&str]) -> Self {
            Self::from(
                value
                    .iter()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<Mapping>>(),
            )
        }
    }

    #[test]
    fn merge() {
        assert_eq!(
            Mappings::from(["1000", "1005-1010:1105-1110", "1010:1110", "1011:1111"].as_slice()),
            Mappings::from(["1000", "1005-1011:1105-1111"].as_slice())
        );
        assert_eq!(
            Mappings::from(["1000-1100", "800-1000:800-1000"].as_slice()).0,
            vec!["800-1100".parse::<Mapping>().unwrap()],
        );
    }
}
