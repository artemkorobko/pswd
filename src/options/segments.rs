use anyhow::Context;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Segments(usize);

impl Segments {
    pub fn as_usize(self) -> usize {
        self.0
    }
}

impl FromStr for Segments {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s
            .parse::<usize>()
            .map(Self)
            .with_context(|| format!("Invalid number of segments: {s}"))?;

        if segments.0 < 1 {
            anyhow::bail!("Minimum number of segments is 1");
        }

        if segments.0 > 3 {
            anyhow::bail!("Maximum number of segments is 3");
        }

        Ok(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", 1)]
    #[case("2", 2)]
    fn read_segments_from_str(#[case] s: &str, #[case] expected: usize) {
        let segments = Segments::from_str(s).unwrap();
        assert_eq!(segments.as_usize(), expected);
    }

    #[test]
    fn do_not_read_segments_from_invalid_str() {
        let err = Segments::from_str("invalid-str").unwrap_err();
        assert_eq!(err.to_string(), "Invalid number of segments: invalid-str");
    }

    #[test]
    fn do_not_read_segments_from_str_less_than_1() {
        let err = Segments::from_str("0").unwrap_err();
        assert_eq!(err.to_string(), "Minimum number of segments is 1");
    }

    #[test]
    fn do_not_read_segments_from_str_greater_than_3() {
        let err = Segments::from_str("4").unwrap_err();
        assert_eq!(err.to_string(), "Maximum number of segments is 3");
    }
}
