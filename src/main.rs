use crate::options::options::Options;
use crate::options::character_type::CharacterType;
use crate::options::password_length::PasswordLength;
use crate::algorithm::algorithm::{GeneratorPtr, Algorithm};
use crate::algorithm::generator_lowercase::GeneratorLowerCase;
use crate::algorithm::generator_uppercase::GeneratorUpperCase;
use crate::algorithm::generator_number::GeneratorNumber;
use crate::algorithm::generator_symbol::GeneratorSymbol;
use crate::algorithm::generator_tokenizer::GeneratorTokenizer;

mod algorithm;
mod options;

fn main() {
    let options = Options::from_args();
    let mut algorithm = Algorithm::new(options.chars.len());
    configure_algorithm_and_generate_passwords(&options, &mut algorithm);
}

fn configure_algorithm_and_generate_passwords(options: &Options, mut algorithm: &mut Algorithm) {
    configure_algorithm_generators(&options.chars, &mut algorithm);
    configure_algorithm_tokenizer(options.tokens, &mut algorithm);
    let average_password_length = calculate_average_password_length(&options.length);
    generate_passwords(options.num, &mut algorithm, average_password_length)
}

fn configure_algorithm_generators(character_types: &Vec<CharacterType>, algorithm: &mut Algorithm) {
    for character_type in character_types {
        let generator = match character_type {
            CharacterType::LowerCase => Box::new(GeneratorLowerCase) as GeneratorPtr,
            CharacterType::UpperCase => Box::new(GeneratorUpperCase) as GeneratorPtr,
            CharacterType::Numbers => Box::new(GeneratorNumber) as GeneratorPtr,
            CharacterType::Symbols => Box::new(GeneratorSymbol) as GeneratorPtr,
        };

        algorithm.add_generator(generator);
    }
}

fn configure_algorithm_tokenizer(tokens_count: usize, algorithm: &mut Algorithm) {
    if tokens_count > 1 {
        let tokenizer = Box::new(GeneratorTokenizer::new(tokens_count));
        algorithm.add_generator(tokenizer);
    }
}

fn calculate_average_password_length(password_length: &PasswordLength) -> usize {
    match password_length {
        PasswordLength::Short => 16,
        PasswordLength::Regular => 24,
        PasswordLength::Long => 32,
    }
}

fn generate_passwords(count: usize, algorithm: &Algorithm, average_password_length: usize) -> () {
    for _ in 0..count {
        let password = algorithm.generate_password(average_password_length);
        println!("{}", password);
    }
}
