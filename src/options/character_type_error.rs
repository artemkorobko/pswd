use std::fmt::{Display, Formatter, Result};

const ERROR_MESSAGE: &str = "Unsupported character type";

#[derive(Debug, PartialEq)]
pub struct UnsupportedCharacterTypeError;

impl Display for UnsupportedCharacterTypeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(ERROR_MESSAGE)
    }
}

#[cfg(test)]
mod tests {
    use crate::options::character_type_error::*;

    #[test]
    fn should_write_error_message() {
        let result = UnsupportedCharacterTypeError.to_string();
        assert_eq!(result, ERROR_MESSAGE);
    }
}
