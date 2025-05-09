pub mod length;
pub mod segments;

pub use crate::options::length::SecretLength;
pub use crate::options::segments::Segments;

/// Secret generator
#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Secret length [10..30] or [s, small, m, medium, l, large]
    #[arg(short, long, default_value = "20")]
    pub length: SecretLength,
    /// Segments count [1..3]
    #[arg(short, long, default_value = "2")]
    pub segments: Segments,
    /// Generate human-readable secret
    #[arg(short, long)]
    pub readable: bool,
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    const APP: &str = "app";

    #[test]
    fn parse_default_args() {
        let args = Args::try_parse_from(&[APP]).unwrap();

        assert_eq!(args.length.as_usize(), 20);
        assert_eq!(args.segments.as_usize(), 1);
    }

    #[rstest::rstest]
    #[case(& [APP, "-l", "20"], 20)]
    #[case(& [APP, "--length", "30"], 30)]
    fn parse_length_arg(#[case] args: &[&str], #[case] expected: usize) {
        let args = Args::try_parse_from(args).unwrap();

        assert_eq!(args.length.as_usize(), expected);
    }

    #[rstest::rstest]
    #[case(& [APP, "-s", "2"], 2)]
    #[case(& [APP, "--segments", "3"], 3)]
    fn parse_segments_arg(#[case] args: &[&str], #[case] expected: usize) {
        let args = Args::try_parse_from(args).unwrap();
        assert_eq!(args.segments.as_usize(), expected);
    }

    #[test]
    fn return_error_on_invalid_length() {
        let result = Args::try_parse_from(&[APP, "-l", "5"]);
        assert!(result.is_err());
    }
}
