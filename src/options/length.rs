use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct SecretLength(usize);

const MINIMUM_LENGTH: usize = 10;
const MEDIUM_LENGTH: usize = 20;
const MAXIMUM_LENGTH: usize = 30;

impl SecretLength {
    pub fn as_usize(self) -> usize {
        self.0
    }

    fn try_from_name(s: &str) -> Option<Self> {
        match s {
            "s" | "small" => Some(Self(MINIMUM_LENGTH)),
            "m" | "medium" => Some(Self(MEDIUM_LENGTH)),
            "l" | "large" => Some(Self(MAXIMUM_LENGTH)),
            _ => None,
        }
    }

    fn try_from_number(s: &str) -> Option<Self> {
        let len = s.parse::<usize>().ok()?;

        if len < MINIMUM_LENGTH {
            return None;
        }

        if len > MAXIMUM_LENGTH {
            return None;
        }

        Some(Self(len))
    }
}

impl FromStr for SecretLength {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_name(s)
            .or_else(|| Self::try_from_number(s))
            .ok_or(anyhow::anyhow!("Unsupported secret length '{s}'"))
    }
}

impl From<SecretLength> for usize {
    fn from(value: SecretLength) -> Self {
        value.as_usize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("s", 10)]
    #[case("small", 10)]
    #[case("m", 20)]
    #[case("medium", 20)]
    #[case("l", 30)]
    #[case("large", 30)]
    fn build_secret_length_from_name(#[case] name: &str, #[case] expected: usize) {
        let len = SecretLength::from_str(name).unwrap();
        assert_eq!(len.as_usize(), expected);
    }

    #[test]
    fn build_secret_length_from_str() {
        let len = SecretLength::from_str("15").unwrap();
        assert_eq!(len.as_usize(), 15);
    }

    #[test]
    fn do_not_build_secret_length_from_invalid_str() {
        let err = SecretLength::from_str("invalid-str").unwrap_err();
        assert_eq!(err.to_string(), "Unsupported secret length 'invalid-str'");
    }

    #[test]
    fn validate_secret_length_minimal_value() {
        let err = SecretLength::from_str("9").unwrap_err();
        assert_eq!(err.to_string(), "Unsupported secret length '9'");

        let len = SecretLength::from_str("10").unwrap();
        assert_eq!(len.as_usize(), 10);
    }

    #[test]
    fn validate_secret_length_maximal_value() {
        let len = SecretLength::from_str("30").unwrap();
        assert_eq!(len.as_usize(), 30);

        let err = SecretLength::from_str("31").unwrap_err();
        assert_eq!(err.to_string(), "Unsupported secret length '31'");
    }
}
