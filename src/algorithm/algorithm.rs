use crate::algorithm::generator::{Generator, calculate_string_length};
use rand::prelude::ThreadRng;
use rand::random;

pub type GeneratorPtr = Box<dyn Generator>;

#[derive(Debug)]
pub struct Algorithm {
    generators: Vec<GeneratorPtr>
}

impl Algorithm {
    pub fn new() -> Self {
        Self { generators: Vec::new() }
    }

    pub fn add_generator(&mut self, generator: GeneratorPtr) -> &mut Self {
        self.generators.push(generator);
        self
    }

    pub fn generate(&self, average_length: usize) -> String {
        if self.generators.is_empty() {
            String::new()
        } else {
            let mut random = rand::thread_rng();
            let string_length = calculate_string_length(average_length, &mut random);
            let mut result = String::with_capacity(string_length);
            let mut iteration = 0;

            for generator in &self.generators {
                result = generator.generate(iteration, result.clone(), &mut random);
                iteration += 1;
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::algorithm::*;
    use crate::algorithm::generator::Generator;
    use rand::prelude::ThreadRng;

    #[test]
    fn should_generate_empty_string_when_no_generators_exists() {
        let algorithm = Algorithm::new();

        let result = algorithm.generate(10);

        assert!(result.is_empty());
    }

    #[test]
    fn should_generate_string_using_generator() {
        let mut algorithm = Algorithm::new();
        algorithm.add_generator(build_test_generator("str1".into()));

        let result = algorithm.generate(10);

        assert_eq!(result, "str1");
    }

    fn build_test_generator(payload: String) -> GeneratorPtr {
        #[derive(Debug)]
        struct Gen<>(String);

        impl Generator for Gen {
            fn generate_string(&self, length: usize, random: &mut ThreadRng) -> String {
                self.0.clone()
            }
        }

        Box::new(Gen { 0: payload })
    }
}
