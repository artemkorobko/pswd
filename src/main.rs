use clap::Parser;

use crate::generator::{CaseModifier, Generator, GeneratorChain, PatternGenerator};

mod generator;
mod options;

const NUMBERS_PATTERN: &str = "1234567890";
const VOWELS_PATTERN: &str = "aeiou";
const CONSONANTS_PATTERN: &str = "bcdfghjklmnpqrstvwxyz";
const SPECIALS_PATTERN: &str = "`!@#$%^&*()_-+={[}]|\\:;\"'<,>.?/";
const SPECIALS_REDUCED_PATTERN: &str = "`!@#$%&_=";

fn main() -> anyhow::Result<()> {
    let args = options::Args::parse();
    let length = args.length.as_usize();
    let segments = args.segments.as_usize();
    let vowels = PatternGenerator::new(VOWELS_PATTERN, 1.0);
    let vowels = CaseModifier::new(Box::new(vowels), 0.1);
    let mut generators = GeneratorChain::new(Box::new(vowels));
    let numbers = PatternGenerator::new(NUMBERS_PATTERN, 0.1);
    generators.and_then(Box::new(numbers));
    let consonants = PatternGenerator::new(CONSONANTS_PATTERN, 0.5);
    let consonants = CaseModifier::new(Box::new(consonants), 0.1);
    generators.and_then(Box::new(consonants));

    let secret = if segments == 1 {
        if !args.readable {
            let specials = PatternGenerator::new(SPECIALS_PATTERN, 0.05);
            generators.start_from(Box::new(specials));
        } else {
            let specials = PatternGenerator::new(SPECIALS_REDUCED_PATTERN, 0.05);
            generators.start_from(Box::new(specials));
        }

        generate_single_segment(length, &mut generators)?
    } else {
        generate_multi_segment(length, segments, &mut generators)?.join("-")
    };

    println!("{secret}");
    Ok(())
}

fn generate_single_segment(mut len: usize, chain: &mut GeneratorChain) -> anyhow::Result<String> {
    let mut secret = String::with_capacity(len);

    while len > 0 {
        let symbol = chain
            .generate()
            .ok_or(anyhow::anyhow!("Failed to generate symbol"))?;
        secret.push(symbol);
        len -= 1;
    }

    Ok(secret)
}

fn generate_multi_segment(
    len: usize,
    count: usize,
    chain: &mut GeneratorChain,
) -> anyhow::Result<Vec<String>> {
    let max_segment_len = len / count;
    let mut total_len = 0;
    let mut segments = Vec::with_capacity(count);

    for _ in 0..count {
        total_len += max_segment_len;

        let segment = if total_len > len {
            let len = core::cmp::min(max_segment_len, len - total_len);
            generate_single_segment(len, chain)
        } else {
            generate_single_segment(max_segment_len, chain)
        }?;

        segments.push(segment);
    }

    Ok(segments)
}
