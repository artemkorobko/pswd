use crate::algorithm::generator::Generator;
use crate::algorithm::initial_password::build_initial_password;

pub type GeneratorPtr = Box<dyn Generator>;

#[derive(Debug)]
pub struct Algorithm {
    generators: Vec<GeneratorPtr>
}

impl Algorithm {
    pub fn new(total_generators: usize) -> Self {
        Self {
            generators: Vec::with_capacity(total_generators)
        }
    }

    pub fn add_generator(&mut self, generator: GeneratorPtr) -> &mut Self {
        self.generators.push(generator);
        self
    }

    pub fn generate_password(&self, average_string_length: usize) -> String {
        let mut random = rand::thread_rng();
        let mut result_string = build_initial_password(average_string_length, &mut random);
        let mut current_iteration_num = 0;

        for generator in &self.generators {
            result_string = generator.generate(current_iteration_num, result_string, &mut random);
            current_iteration_num += 1;
        }

        result_string
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::ThreadRng;
    use crate::algorithm::algorithm::*;
    use crate::algorithm::generator::Generator;

    #[test]
    fn should_create_algorithm_with_predefined_amount_of_generators() {
        let total_generators = 10;

        let algorithm = Algorithm::new(total_generators);

        assert_eq!(algorithm.generators.capacity(), total_generators);
    }

    #[test]
    fn should_generate_empty_string_when_no_generators_exists() {
        let total_generators = 0;
        let average_string_length = 10;
        let algorithm = Algorithm::new(total_generators);

        let result = algorithm.generate_password(average_string_length);

        assert!(result.is_empty());
    }

    #[test]
    fn should_generate_string_using_generator() {
        let expected_char = 'a';
        let total_generators = 0;
        let average_string_length = 10;
        let mut algorithm = Algorithm::new(total_generators);
        algorithm.add_generator(build_test_generator(expected_char));

        let result = algorithm.generate_password(average_string_length);

        assert!(!result.is_empty());
        for char in result.chars() {
            assert_eq!(char, expected_char);
        }
    }

    fn build_test_generator(payload: char) -> GeneratorPtr {
        #[derive(Debug)]
        struct TestGenerator(char);

        impl Generator for TestGenerator {
            fn generate_character(&self, _: &mut ThreadRng) -> char {
                self.0
            }
        }

        Box::new(TestGenerator { 0: payload })
    }
}
