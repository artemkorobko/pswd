use std::str::FromStr;

#[derive(Debug)]
pub enum PasswordLength {
    Short,
    Regular,
    Long
}

impl PasswordLength {
    pub fn variants() -> Vec<&'static str> {
        vec!["short", "s", "regular", "r", "long", "l"]
    }
}

impl FromStr for PasswordLength {
    type Err = (u8);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "short" | "s" => Ok(PasswordLength::Short),
            "regular" | "r" => Ok(PasswordLength::Regular),
            "long" | "l" => Ok(PasswordLength::Long),
            _ => Err(0)
        }
    }
}
