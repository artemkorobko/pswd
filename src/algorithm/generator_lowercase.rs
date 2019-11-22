use rand::Rng;
use rand::rngs::ThreadRng;
use crate::algorithm::generator::Generator;

const LOWERCASE_CHARACTER_CODE_A: u8 = 97;
const LOWERCASE_CHARACTER_CODE_Z: u8 = 122;

#[derive(Debug)]
pub struct GeneratorLowerCase;

impl Generator for GeneratorLowerCase {
    fn generate_character(&self, random: &mut ThreadRng) -> char {
        random.gen_range(LOWERCASE_CHARACTER_CODE_A, LOWERCASE_CHARACTER_CODE_Z + 1).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::generator_lowercase::*;
    use crate::algorithm::generator::Generator;

    #[test]
    fn should_generate_random_character_in_range() {
        let mut random = rand::thread_rng();
        let generator = GeneratorLowerCase;

        let result = generator.generate_character(&mut random) as u8;

        assert!(result >= LOWERCASE_CHARACTER_CODE_A && result <= LOWERCASE_CHARACTER_CODE_Z);
    }
}
