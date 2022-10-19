use std::str::FromStr;

use structopt::StructOpt;

/// Secure password generator
#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Args {
    /// Password length
    #[structopt(
        short,
        long,
        possible_values = &["small", "s", "medium", "m", "large", "l"],
        default_value = "s",
    )]
    pub length: PasswordLength,
    /// Number of password tokens separated by dash
    #[structopt(
        short,
        long,
        possible_values = &["1", "2", "3"],
        default_value = "3",
    )]
    pub tokens: usize,
    /// Copy password to clipboard
    #[structopt(short, long)]
    pub clipboard: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PasswordLength {
    Small,
    Medium,
    Large,
}

impl FromStr for PasswordLength {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" | "small" => Ok(Self::Small),
            "m" | "medium" => Ok(Self::Medium),
            "l" | "large" => Ok(Self::Large),
            _ => anyhow::bail!("Unsupported value '{s}'"),
        }
    }
}

impl From<PasswordLength> for usize {
    fn from(value: PasswordLength) -> Self {
        match value {
            PasswordLength::Small => 10,
            PasswordLength::Medium => 15,
            PasswordLength::Large => 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const APP: &str = "app";

    #[test]
    fn return_default_args() {
        let args = Args::from_iter(&[APP]);

        assert_eq!(args.length, PasswordLength::Small);
        assert_eq!(args.tokens, 3);
        assert_eq!(args.clipboard, false);
    }

    #[rstest::rstest]
    #[case("l", PasswordLength::Large, 20)]
    #[case("large", PasswordLength::Large, 20)]
    #[case("m", PasswordLength::Medium, 15)]
    #[case("medium", PasswordLength::Medium, 15)]
    #[case("s", PasswordLength::Small, 10)]
    #[case("small", PasswordLength::Small, 10)]
    fn parse_length_arg(#[case] param: &str, #[case] expected: PasswordLength, #[case] len: usize) {
        let args = Args::from_iter(&[APP, "-l", param]);
        assert_eq!(args.length, expected);
        assert_eq!(usize::from(args.length), len);
    }

    #[test]
    fn parse_invalid_length_arg() {
        let args = Args::from_iter_safe(&[APP, "-l", "invalid length"]);
        assert!(args.is_err());
    }

    #[rstest::rstest]
    #[case("1", 1)]
    #[case("2", 2)]
    #[case("3", 3)]
    fn parse_tokens_arg(#[case] tokens: &str, #[case] expected: usize) {
        let args = Args::from_iter(&[APP, "-t", tokens]);
        assert_eq!(args.tokens, expected);
    }

    #[test]
    fn parse_invalid_tokes_arg() {
        let args = Args::from_iter_safe(&[APP, "-t", "invalid tokens"]);
        assert!(args.is_err());
    }

    #[rstest::rstest]
    #[case("-c", true)]
    fn parse_clipboard_flag(#[case] ignore_clipboard: &str, #[case] expected: bool) {
        let args = Args::from_iter(&[APP, ignore_clipboard]);
        assert_eq!(args.clipboard, expected);
    }
}
