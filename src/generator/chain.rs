use std::collections::LinkedList;

use crate::generator::Generator;

pub struct GeneratorChain {
    generators: LinkedList<Box<dyn Generator>>,
    fallback: Box<dyn Generator>,
}

impl GeneratorChain {
    pub fn new(fallback: Box<dyn Generator>) -> Self {
        Self {
            generators: LinkedList::new(),
            fallback,
        }
    }

    pub fn start_from(&mut self, generator: Box<dyn Generator>) {
        self.generators.push_front(generator);
    }

    pub fn and_then(&mut self, generator: Box<dyn Generator>) {
        self.generators.push_back(generator);
    }
}

impl Generator for GeneratorChain {
    fn generate(&mut self) -> Option<char> {
        for generator in self.generators.iter_mut() {
            let symbol = generator.generate();
            if symbol.is_some() {
                return symbol;
            }
        }

        self.fallback.generate()
    }
}
