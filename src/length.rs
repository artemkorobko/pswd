use anyhow::Context;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct SecretLength(usize);

impl SecretLength {
    pub fn as_usize(self) -> usize {
        self.0
    }
}

impl FromStr for SecretLength {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s
            .parse::<usize>()
            .with_context(|| anyhow::anyhow!("Invalid secret length"))?;
        SecretLength::try_from(len)
    }
}

impl TryFrom<usize> for SecretLength {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let minimum_length = 10;
        if value < minimum_length {
            return Err(anyhow::anyhow!(
                "Secret length is too small. The minimal length is {minimum_length}."
            ));
        }

        let maximum_length = 30;
        if value > maximum_length {
            return Err(anyhow::anyhow!(
                "Secret length is too large. The maximal length is {maximum_length}."
            ));
        }

        Ok(Self(value))
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

    #[test]
    fn build_secret_length_from_str() {
        let len = SecretLength::from_str("15").unwrap();
        assert_eq!(len.as_usize(), 15);
    }

    #[test]
    fn build_secret_length_from_usize() {
        let len = SecretLength::try_from(15).unwrap();
        assert_eq!(len.as_usize(), 15);
    }

    #[test]
    fn do_not_build_secret_length_from_invalid_str() {
        let err = SecretLength::from_str("invalid-str").unwrap_err();
        assert_eq!(err.to_string(), "Invalid secret length");
    }

    #[test]
    fn validate_secret_length_minimal_value() {
        let err = SecretLength::from_str("9").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Secret length is too small. The minimal length is 10."
        );

        let len = SecretLength::from_str("10").unwrap();
        assert_eq!(len.as_usize(), 10);
    }

    #[test]
    fn validate_secret_length_maximal_value() {
        let len = SecretLength::from_str("30").unwrap();
        assert_eq!(len.as_usize(), 30);

        let err = SecretLength::from_str("31").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Secret length is too large. The maximal length is 30."
        );
    }
}
