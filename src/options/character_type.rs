use std::str::FromStr;
use crate::options::character_type_error::UnsupportedCharacterTypeError;

const LOWER_CASE_LONG_PARAMETER_KEY: &str = "lower-case";
const LOWER_CASE_SHORT_PARAMETER_KEY: &str = "l";
const UPPER_CASE_LONG_PARAMETER_KEY: &str = "upper-case";
const UPPER_CASE_SHORT_PARAMETER_KEY: &str = "u";
const NUMBERS_LONG_PARAMETER_KEY: &str = "numbers";
const NUMBERS_SHORT_PARAMETER_KEY: &str = "n";
const SYMBOLS_LONG_PARAMETER_KEY: &str = "symbols";
const SYMBOLS_SHORT_PARAMETER_KEY: &str = "s";

#[derive(Debug, PartialEq)]
pub enum CharacterType {
    LowerCase,
    UpperCase,
    Numbers,
    Symbols,
}

impl CharacterType {
    pub fn variants() -> Vec<&'static str> {
        vec![
            LOWER_CASE_LONG_PARAMETER_KEY, LOWER_CASE_SHORT_PARAMETER_KEY,
            UPPER_CASE_LONG_PARAMETER_KEY, UPPER_CASE_SHORT_PARAMETER_KEY,
            NUMBERS_LONG_PARAMETER_KEY, NUMBERS_SHORT_PARAMETER_KEY,
            SYMBOLS_LONG_PARAMETER_KEY, SYMBOLS_SHORT_PARAMETER_KEY
        ]
    }
}

impl FromStr for CharacterType {
    type Err = UnsupportedCharacterTypeError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_ref() {
            LOWER_CASE_LONG_PARAMETER_KEY | LOWER_CASE_SHORT_PARAMETER_KEY => Ok(CharacterType::LowerCase),
            UPPER_CASE_LONG_PARAMETER_KEY | UPPER_CASE_SHORT_PARAMETER_KEY => Ok(CharacterType::UpperCase),
            NUMBERS_LONG_PARAMETER_KEY | NUMBERS_SHORT_PARAMETER_KEY => Ok(CharacterType::Numbers),
            SYMBOLS_LONG_PARAMETER_KEY | SYMBOLS_SHORT_PARAMETER_KEY => Ok(CharacterType::Symbols),
            _ => Err(UnsupportedCharacterTypeError)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::options::character_type::*;
    use crate::options::character_type_error::UnsupportedCharacterTypeError;

    #[test]
    fn should_return_command_variants() {
        let variants = CharacterType::variants();

        assert_eq!(variants, vec![
            LOWER_CASE_LONG_PARAMETER_KEY, LOWER_CASE_SHORT_PARAMETER_KEY,
            UPPER_CASE_LONG_PARAMETER_KEY, UPPER_CASE_SHORT_PARAMETER_KEY,
            NUMBERS_LONG_PARAMETER_KEY, NUMBERS_SHORT_PARAMETER_KEY,
            SYMBOLS_LONG_PARAMETER_KEY, SYMBOLS_SHORT_PARAMETER_KEY
        ]);
    }

    #[test]
    fn should_parse_lower_case_long_parameter() {
        let result = CharacterType::from_str(LOWER_CASE_LONG_PARAMETER_KEY);
        assert_eq!(result.unwrap(), CharacterType::LowerCase);
    }

    #[test]
    fn should_parse_lower_case_short_parameter() {
        let result = CharacterType::from_str(LOWER_CASE_SHORT_PARAMETER_KEY);
        assert_eq!(result.unwrap(), CharacterType::LowerCase);
    }

    #[test]
    fn should_parse_upper_case_long_parameter() {
        let result = CharacterType::from_str(UPPER_CASE_LONG_PARAMETER_KEY);
        assert_eq!(result.unwrap(), CharacterType::UpperCase);
    }

    #[test]
    fn should_parse_upper_case_short_parameter() {
        let result = CharacterType::from_str(UPPER_CASE_SHORT_PARAMETER_KEY);
        assert_eq!(result.unwrap(), CharacterType::UpperCase);
    }

    #[test]
    fn should_parse_numbers_long_parameter() {
        let result = CharacterType::from_str(NUMBERS_LONG_PARAMETER_KEY);
        assert_eq!(result.unwrap(), CharacterType::Numbers);
    }

    #[test]
    fn should_parse_numbers_short_parameter() {
        let result = CharacterType::from_str(NUMBERS_SHORT_PARAMETER_KEY);
        assert_eq!(result.unwrap(), CharacterType::Numbers);
    }

    #[test]
    fn should_parse_symbols_long_parameter() {
        let result = CharacterType::from_str(SYMBOLS_LONG_PARAMETER_KEY);
        assert_eq!(result.unwrap(), CharacterType::Symbols);
    }

    #[test]
    fn should_parse_symbols_short_parameter() {
        let result = CharacterType::from_str(SYMBOLS_SHORT_PARAMETER_KEY);
        assert_eq!(result.unwrap(), CharacterType::Symbols);
    }

    #[test]
    fn should_return_error_when_type_is_not_supported() {
        let result = CharacterType::from_str("unsupported");
        assert_eq!(result.err().unwrap(), UnsupportedCharacterTypeError);
    }
}
