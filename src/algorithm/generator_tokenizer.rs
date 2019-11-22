use rand::Rng;
use rand::prelude::ThreadRng;
use itertools::Itertools;
use crate::algorithm::generator::Generator;

const TOKEN_SEPARATOR: &str = "-";

#[derive(Debug)]
pub struct GeneratorTokenizer {
    tokens: usize
}

impl GeneratorTokenizer {
    pub fn new(tokens: usize) -> Self {
        Self { tokens }
    }
}

impl Generator for GeneratorTokenizer {
    fn generate(&self, _: usize, initial_value: String, _: &mut ThreadRng) -> String {
        let bytes = initial_value.into_bytes();
        bytes.chunks(bytes.len() / self.tokens)
            .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
            .join(TOKEN_SEPARATOR)
    }

    fn generate_character(&self, _: &mut ThreadRng) -> char {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::generator_tokenizer::*;
    use crate::algorithm::generator::Generator;

    #[test]
    fn should_generate_random_character_in_range() {
        let mut random = rand::thread_rng();
        let generator = GeneratorTokenizer::new(2);

        let result = generator.generate(0, "1234567890".into(), &mut random);

        assert_eq!(result, "12345-67890");
    }
}
