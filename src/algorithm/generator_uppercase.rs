use rand::Rng;
use rand::rngs::ThreadRng;
use crate::algorithm::generator::Generator;

const UPPERCASE_CHARACTER_CODE_A: u8 = 65;
const UPPERCASE_CHARACTER_CODE_Z: u8 = 90;

#[derive(Debug)]
pub struct GeneratorUpperCase;

impl Generator for GeneratorUpperCase {
    fn generate_character(&self, random: &mut ThreadRng) -> char {
        random.gen_range(UPPERCASE_CHARACTER_CODE_A, UPPERCASE_CHARACTER_CODE_Z + 1).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::generator_uppercase::*;
    use crate::algorithm::generator::Generator;

    #[test]
    fn should_generate_random_character_in_range() {
        let mut random = rand::thread_rng();
        let generator = GeneratorUpperCase;

        let result = generator.generate_character(&mut random) as u8;

        assert!(result >= UPPERCASE_CHARACTER_CODE_A && result <= UPPERCASE_CHARACTER_CODE_Z);
    }
}
