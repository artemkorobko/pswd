use crate::algorithm::generator::Generator;
use rand::rngs::ThreadRng;
use rand::Rng;

const SYMBOLS_TABLE: [char; 29] = [
    '!', '#', '$', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?',
    '@', '[', ']', '^', '_', '`', '{', '|', '}', '~',
];

#[derive(Debug)]
pub struct GeneratorSymbol;

impl Generator for GeneratorSymbol {
    fn generate_character(&self, random: &mut ThreadRng) -> char {
        let symbol_index = random.gen_range(0, SYMBOLS_TABLE.len());
        SYMBOLS_TABLE[symbol_index]
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::generator::Generator;
    use crate::algorithm::generator_symbol::*;

    #[test]
    fn should_generate_random_character_in_range() {
        let mut random = rand::thread_rng();
        let generator = GeneratorSymbol;

        let result = generator.generate_character(&mut random);

        assert!(SYMBOLS_TABLE.contains(&result));
    }
}
