use rand::Rng;
use rand::prelude::ThreadRng;
use crate::algorithm::generator::Generator;

const NUMBER_CODE_0: u8 = 48;
const NUMBER_CODE_9: u8 = 57;

#[derive(Debug)]
pub struct GeneratorNumber;

impl Generator for GeneratorNumber {
    fn generate_character(&self, random: &mut ThreadRng) -> char {
        random.gen_range(NUMBER_CODE_0, NUMBER_CODE_9 + 1).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::generator_number::*;
    use crate::algorithm::generator::Generator;

    #[test]
    fn should_generate_random_number_in_range() {
        let mut random = rand::thread_rng();
        let generator = GeneratorNumber;

        let result = generator.generate_character(&mut random) as u8;

        assert!(result >= NUMBER_CODE_0 && result <= NUMBER_CODE_9);
    }
}
