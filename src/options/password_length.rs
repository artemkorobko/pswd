use std::str::FromStr;
use crate::options::password_length_error::UnsupportedPasswordLengthError;

const SHORT_LENGTH_LONG_PARAMETER_KEY: &str = "short";
const SHORT_LENGTH_SHORT_PARAMETER_KEY: &str = "s";
const REGULAR_LENGTH_LONG_PARAMETER_KEY: &str = "regular";
const REGULAR_LENGTH_SHORT_PARAMETER_KEY: &str = "r";
const LONG_LENGTH_LONG_PARAMETER_KEY: &str = "long";
const LONG_LENGTH_SHORT_PARAMETER_KEY: &str = "l";

#[derive(Debug, PartialEq)]
pub enum PasswordLength {
    Short,
    Regular,
    Long,
}

impl PasswordLength {
    pub fn variants() -> Vec<&'static str> {
        vec![
            SHORT_LENGTH_LONG_PARAMETER_KEY, SHORT_LENGTH_SHORT_PARAMETER_KEY,
            REGULAR_LENGTH_LONG_PARAMETER_KEY, REGULAR_LENGTH_SHORT_PARAMETER_KEY,
            LONG_LENGTH_LONG_PARAMETER_KEY, LONG_LENGTH_SHORT_PARAMETER_KEY
        ]
    }
}

impl FromStr for PasswordLength {
    type Err = UnsupportedPasswordLengthError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_ref() {
            SHORT_LENGTH_LONG_PARAMETER_KEY | SHORT_LENGTH_SHORT_PARAMETER_KEY => Ok(PasswordLength::Short),
            REGULAR_LENGTH_LONG_PARAMETER_KEY | REGULAR_LENGTH_SHORT_PARAMETER_KEY => Ok(PasswordLength::Regular),
            LONG_LENGTH_LONG_PARAMETER_KEY | LONG_LENGTH_SHORT_PARAMETER_KEY => Ok(PasswordLength::Long),
            _ => Err(UnsupportedPasswordLengthError)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::options::password_length::*;
    use crate::options::password_length_error::UnsupportedPasswordLengthError;

    #[test]
    fn should_return_command_variants() {
        let variants = PasswordLength::variants();

        assert_eq!(variants, vec![
            SHORT_LENGTH_LONG_PARAMETER_KEY, SHORT_LENGTH_SHORT_PARAMETER_KEY,
            REGULAR_LENGTH_LONG_PARAMETER_KEY, REGULAR_LENGTH_SHORT_PARAMETER_KEY,
            LONG_LENGTH_LONG_PARAMETER_KEY, LONG_LENGTH_SHORT_PARAMETER_KEY
        ]);
    }

    #[test]
    fn should_parse_short_length_long_parameter() {
        let result = PasswordLength::from_str(SHORT_LENGTH_LONG_PARAMETER_KEY);
        assert_eq!(result.unwrap(), PasswordLength::Short);
    }

    #[test]
    fn should_parse_short_length_short_parameter() {
        let result = PasswordLength::from_str(SHORT_LENGTH_SHORT_PARAMETER_KEY);
        assert_eq!(result.unwrap(), PasswordLength::Short);
    }

    #[test]
    fn should_parse_regular_length_long_parameter() {
        let result = PasswordLength::from_str(REGULAR_LENGTH_LONG_PARAMETER_KEY);
        assert_eq!(result.unwrap(), PasswordLength::Regular);
    }

    #[test]
    fn should_parse_regular_length_short_parameter() {
        let result = PasswordLength::from_str(REGULAR_LENGTH_SHORT_PARAMETER_KEY);
        assert_eq!(result.unwrap(), PasswordLength::Regular);
    }

    #[test]
    fn should_parse_long_length_long_parameter() {
        let result = PasswordLength::from_str(LONG_LENGTH_LONG_PARAMETER_KEY);
        assert_eq!(result.unwrap(), PasswordLength::Long);
    }

    #[test]
    fn should_parse_long_length_short_parameter() {
        let result = PasswordLength::from_str(LONG_LENGTH_SHORT_PARAMETER_KEY);
        assert_eq!(result.unwrap(), PasswordLength::Long);
    }

    #[test]
    fn should_return_error_when_length_is_not_supported() {
        let result = PasswordLength::from_str("unsupported");
        assert_eq!(result.err().unwrap(), UnsupportedPasswordLengthError);
    }
}
