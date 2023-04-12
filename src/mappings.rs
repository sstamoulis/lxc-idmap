mod mapping;

use std::ops::Deref;

pub use mapping::Mapping;

#[derive(Debug, PartialEq)]
pub struct Mappings(Vec<Mapping>);

impl Mappings {
    fn merge_overlapping(&mut self) {
        self.0.sort_unstable_by_key(|m| m.host_range.start);
        let mut iter = std::mem::take(&mut self.0).into_iter();
        let stack = &mut self.0;
        if let Some(mut last_m) = iter.next() {
            for m in iter {
                if
                // is host_range overlapping?
                m.host_range.start <= last_m.host_range.end &&
                // is container_range overlapping?
                m.container_range.start >= last_m.container_range.start &&
                m.container_range.start <= last_m.container_range.end &&
                // are overlap start offsets equal between host and container ranges?
                m.host_range.start - last_m.host_range.start == m.container_range.start - last_m.container_range.start &&
                // are overlap end offsets equal between host and container ranges?
                last_m.host_range.end.cmp(&m.host_range.end) == last_m.container_range.end.cmp(&m.container_range.end) &&
                last_m.host_range.end.abs_diff(m.host_range.end) == last_m.container_range.end.abs_diff(m.container_range.end)
                {
                    last_m.host_range.end = last_m.host_range.end.max(m.host_range.end);
                    last_m.container_range.end =
                        last_m.container_range.end.max(m.container_range.end);
                } else {
                    stack.push(last_m);
                    last_m = m;
                }
            }
            stack.push(last_m);
        }
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
            Mappings::from(["1000-1100", "900-1000:800-1000"].as_slice()).0,
            vec![
                "900-1000:800-1000".parse::<Mapping>().unwrap(),
                "1000-1100".parse().unwrap(),
            ],
        );
    }
}
