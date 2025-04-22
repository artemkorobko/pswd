pub mod case;
pub mod chain;
pub mod pattern;

pub use case::CaseModifier;
pub use chain::GeneratorChain;
pub use pattern::PatternGenerator;

pub trait Generator {
    fn generate(&mut self) -> Option<char>;
}
