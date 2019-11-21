use crate::algorithm::generator::Generator;

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

    pub fn generate(&self) -> String {
        if self.generators.is_empty() {
            String::new()
        } else {
            let mut thread_rng = rand::thread_rng();
            let mut result = String::new();
            let mut iteration = 0;

            for generator in &self.generators {
                result = generator.generate(iteration, result.clone(), &mut thread_rng);
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

        let result = algorithm.generate();

        assert!(result.is_empty());
    }

    #[test]
    fn should_generate_string_using_generator() {
        let mut algorithm = Algorithm::new();
        algorithm.add_generator(build_test_generator("str1"))
            .add_generator(build_test_generator("str2"));

        let result = algorithm.generate();

        assert_eq!(result, "-0-str1-1-str2");
    }

    fn build_test_generator(payload: &'static str) -> GeneratorPtr {
        #[derive(Debug)]
        struct Gen<'a>(&'a str);

        impl Generator for Gen<'_> {
            fn generate(&self, sequence_number: usize, initial_value: String, _: &mut ThreadRng) -> String {
                format!("{}-{}-{}", initial_value, sequence_number, self.0)
            }
        }

        Box::new(Gen { 0: payload.clone() })
    }
}
