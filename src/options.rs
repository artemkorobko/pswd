use std::str::FromStr;

/// Secure password generator
#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Password length [possible values: s, small, m, medium, l, large]
    #[arg(short, long, default_value = "s")]
    pub length: PasswordLength,
    /// Number of password tokens separated by dash
    #[arg(short, long, default_value = "3")]
    pub tokens: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PasswordLength {
    Small,
    Medium,
    Large,
    Custom(usize),
}

impl PasswordLength {
    fn try_from_name(s: &str) -> Option<Self> {
        match s {
            "s" | "small" => Some(Self::Small),
            "m" | "medium" => Some(Self::Medium),
            "l" | "large" => Some(Self::Large),
            _ => None,
        }
    }

    fn try_from_number(s: &str) -> Option<Self> {
        s.parse::<usize>().map(Self::Custom).ok()
    }
}

impl FromStr for PasswordLength {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_name(s)
            .or_else(|| Self::try_from_number(s))
            .ok_or(anyhow::anyhow!("Unsupported value '{s}'"))
    }
}

impl From<PasswordLength> for usize {
    fn from(value: PasswordLength) -> Self {
        match value {
            PasswordLength::Small => 10,
            PasswordLength::Medium => 15,
            PasswordLength::Large => 20,
            PasswordLength::Custom(len) => len,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    const APP: &str = "app";

    #[test]
    fn return_default_args() {
        let args = Args::try_parse_from(&[APP]).expect("Should parse empty arguments");

        assert_eq!(args.length, PasswordLength::Small);
        assert_eq!(args.tokens, 3);
    }

    #[rstest::rstest]
    #[case("25", PasswordLength::Custom(25), 25)]
    #[case("l", PasswordLength::Large, 20)]
    #[case("large", PasswordLength::Large, 20)]
    #[case("m", PasswordLength::Medium, 15)]
    #[case("medium", PasswordLength::Medium, 15)]
    #[case("s", PasswordLength::Small, 10)]
    #[case("small", PasswordLength::Small, 10)]
    fn parse_length_arg(#[case] param: &str, #[case] expected: PasswordLength, #[case] len: usize) {
        let args = Args::try_parse_from(&[APP, "-l", param])
            .expect("Should parse password length argument");

        assert_eq!(args.length, expected);
        assert_eq!(usize::from(args.length), len);
    }

    #[test]
    fn parse_invalid_length_arg() {
        let args = Args::try_parse_from(&[APP, "-l", "invalid length"]);

        assert!(args.is_err());
    }

    #[rstest::rstest]
    #[case("1", 1)]
    #[case("2", 2)]
    #[case("3", 3)]
    fn parse_tokens_arg(#[case] tokens: &str, #[case] expected: usize) {
        let args = Args::try_parse_from(&[APP, "-t", tokens])
            .expect("Should parse password length argument as tokens count");

        assert_eq!(args.tokens, expected);
    }

    #[test]
    fn parse_invalid_tokes_arg() {
        let args = Args::try_parse_from(&[APP, "-t", "invalid tokens count"]);

        assert!(args.is_err());
    }
}
