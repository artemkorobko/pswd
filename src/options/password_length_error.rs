use std::fmt::{Display, Formatter, Result};

const ERROR_MESSAGE: &str = "Unsupported password length";

#[derive(Debug, PartialEq)]
pub struct UnsupportedPasswordLengthError;

impl Display for UnsupportedPasswordLengthError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(ERROR_MESSAGE)
    }
}

#[cfg(test)]
mod tests {
    use crate::options::password_length_error::*;

    #[test]
    fn should_write_error_message() {
        let result = UnsupportedPasswordLengthError.to_string();
        assert_eq!(result, ERROR_MESSAGE);
    }
}
