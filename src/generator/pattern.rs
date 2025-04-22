use rand::prelude::*;

use crate::generator::Generator;

pub struct PatternGenerator<'a> {
    probability: f64,
    pattern: &'a str,
    random: ThreadRng,
}

impl<'a> PatternGenerator<'a> {
    pub fn new(pattern: &'a str, probability: f64) -> Self {
        Self {
            pattern,
            random: rand::rng(),
            probability,
        }
    }
}

impl Generator for PatternGenerator<'_> {
    fn generate(&mut self) -> Option<char> {
        let should_generate = self.random.random_bool(self.probability);

        if should_generate {
            return self.pattern.chars().choose(&mut self.random);
        };

        None
    }
}
