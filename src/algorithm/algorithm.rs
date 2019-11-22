use rand::Rng;
use rand::prelude::ThreadRng;
use crate::algorithm::generator::Generator;

const AVERAGE_LENGTH_BOUNDS: usize = 2;

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
        let mut result_string = build_result_string(average_string_length, &mut random);
        let mut current_iteration_num = 0;

        for generator in &self.generators {
            result_string = generator.generate(current_iteration_num, result_string, &mut random);
            current_iteration_num += 1;
        }

        result_string
    }
}

fn build_result_string(average_string_length: usize, mut random: &mut ThreadRng) -> String {
    let string_length = calculate_string_length(average_string_length, &mut random);
    String::with_capacity(string_length)
}

fn calculate_string_length(average_length: usize, random: &mut ThreadRng) -> usize {
    let min_length = average_length - AVERAGE_LENGTH_BOUNDS;
    let max_length = average_length + AVERAGE_LENGTH_BOUNDS;
    random.gen_range(min_length, max_length)
}

#[cfg(test)]
mod tests {
    use rand::prelude::ThreadRng;
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
        let total_generators = 0;
        let average_string_length = 10;
        let mut algorithm = Algorithm::new(total_generators);
        algorithm.add_generator(build_test_generator());

        let result = algorithm.generate_password(average_string_length);

        assert!(result.len() > average_string_length - AVERAGE_LENGTH_BOUNDS);
        assert!(result.len() < average_string_length + AVERAGE_LENGTH_BOUNDS);
    }

    fn build_test_generator() -> GeneratorPtr {
        #[derive(Debug)]
        struct TestGenerator<>();

        impl Generator for TestGenerator {
            fn generate_character(&self, _: &mut ThreadRng) -> char {
                'a'
            }
        }

        Box::new(TestGenerator {})
    }
}
