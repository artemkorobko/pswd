use std::str::FromStr;

#[derive(Debug)]
pub enum CharacterType {
    LowerCase,
    UpperCase,
    Numbers,
    SpecialCharacters
}

impl CharacterType {
    pub fn variants() -> Vec<&'static str> {
        vec!["lower-case", "l", "upper-case", "u", "numbers", "n", "special-chars", "s"]
    }
}

impl FromStr for CharacterType {
    type Err = (u8);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "lower-case" | "l" => Ok(CharacterType::LowerCase),
            "upper-case" | "u" => Ok(CharacterType::UpperCase),
            "numbers" | "n" => Ok(CharacterType::Numbers),
            "special-chars" | "s" => Ok(CharacterType::SpecialCharacters),
            _ => Err(0)
        }
    }
}
