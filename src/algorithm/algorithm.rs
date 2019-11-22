use crate::algorithm::generator::{Generator, calculate_string_length};
use rand::prelude::ThreadRng;
use rand::random;

pub type GeneratorPtr = Box<dyn Generator>;

#[derive(Debug)]
pub struct Algorithm {
    generators: Vec<GeneratorPtr>
}

impl Algorithm {
    pub fn new(total_generators: usize) -> Self {
        Self { generators: Vec::with_capacity(total_generators) }
    }

    pub fn add_generator(&mut self, generator: GeneratorPtr) -> &mut Self {
        self.generators.push(generator);
        self
    }

    pub fn generate(&self, average_length: usize) -> String {
        let mut random = rand::thread_rng();
        let string_length = calculate_string_length(average_length, &mut random);
        let mut result = String::with_capacity(string_length);
        let mut iteration = 0;

        for generator in &self.generators {
            result = generator.generate(iteration, result, &mut random);
            iteration += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::algorithm::*;
    use crate::algorithm::generator::Generator;
    use rand::prelude::ThreadRng;

    #[test]
    fn should_generate_empty_string_when_no_generators_exists() {
        let algorithm = Algorithm::new(0);

        let result = algorithm.generate(10);

        assert!(result.is_empty());
        assert_eq!(algorithm.generators.capacity(), 0);
    }

    #[test]
    fn should_generate_string_using_generator() {
        let average_string_length = 10;
        let mut algorithm = Algorithm::new(1);
        algorithm.add_generator(build_test_generator());

        let result = algorithm.generate(average_string_length);

        assert!(result.len() > average_string_length - 2);
        assert!(result.len() < average_string_length + 2);
        assert_eq!(algorithm.generators.capacity(), 1);
    }

    fn build_test_generator() -> GeneratorPtr {
        #[derive(Debug)]
        struct Gen<>();

        impl Generator for Gen {
            fn generate_character(&self, _: &mut ThreadRng) -> char {
                'a'
            }
        }

        Box::new(Gen {})
    }
}
