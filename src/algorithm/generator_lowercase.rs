use rand::Rng;
use rand::prelude::ThreadRng;
use crate::algorithm::generator::Generator;

#[derive(Debug)]
pub struct GeneratorLowerCase;

impl Generator for GeneratorLowerCase {
    fn generate_character(&self, random: &mut ThreadRng) -> char {
        random.gen_range(97, 123).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::generator_lowercase::GeneratorLowerCase;
    use crate::algorithm::generator::Generator;

    #[test]
    fn should_generate_random_character() {
        let mut random = rand::thread_rng();
        let generator = GeneratorLowerCase;

        let result = generator.generate_character(&mut random) as u8;

        assert!(result > 96 && result < 123);
    }
}
