use rand::prelude::*;

use crate::generator::Generator;

pub struct CaseModifier {
    inner: Box<dyn Generator>,
    probability: f64,
    random: ThreadRng,
}

impl CaseModifier {
    pub fn new(inner: Box<dyn Generator>, probability: f64) -> Self {
        Self {
            inner,
            random: rand::rng(),
            probability,
        }
    }
}

impl Generator for CaseModifier {
    fn generate(&mut self) -> Option<char> {
        if let Some(symbol) = self.inner.generate() {
            let modify = self.random.random_bool(self.probability);

            return if modify {
                symbol.to_uppercase().next()
            } else {
                symbol.to_lowercase().next()
            };
        }

        None
    }
}
