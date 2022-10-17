use std::str::FromStr;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Args {
    #[structopt(
        short,
        long,
        possible_values = &["short", "s", "medium", "m", "long", "l"],
        default_value = "l",
        help = "Password length"
    )]
    pub length: PasswordLength,
    #[structopt(
        short,
        long,
        possible_values = &["1", "2", "3"],
        default_value = "3",
        help = "Number of password tokens separated by dash"
    )]
    pub tokens: usize,
    #[structopt(
        short,
        long,
        help = "Doesn't copy last generated password to the clipboard"
    )]
    pub ignore_clipboard: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PasswordLength {
    Short,
    Medium,
    Long,
}

impl FromStr for PasswordLength {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" | "short" => Ok(Self::Short),
            "m" | "medium" => Ok(Self::Medium),
            "l" | "long" => Ok(Self::Long),
            _ => anyhow::bail!("Unsupported value '{s}'"),
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

        assert_eq!(args.length, PasswordLength::Long);
        assert_eq!(args.tokens, 3);
        assert_eq!(args.ignore_clipboard, false);
    }

    #[rstest::rstest]
    #[case("l", PasswordLength::Long)]
    #[case("long", PasswordLength::Long)]
    #[case("m", PasswordLength::Medium)]
    #[case("medium", PasswordLength::Medium)]
    #[case("s", PasswordLength::Short)]
    #[case("short", PasswordLength::Short)]
    fn parse_length_arg(#[case] length: &str, #[case] expected: PasswordLength) {
        let args = Args::from_iter(&[APP, "-l", length]);
        assert_eq!(args.length, expected);
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
    #[case("-i", true)]
    fn parse_ignore_clipboard_flag(#[case] ignore_clipboard: &str, #[case] expected: bool) {
        let args = Args::from_iter(&[APP, ignore_clipboard]);
        assert_eq!(args.ignore_clipboard, expected);
    }
}
