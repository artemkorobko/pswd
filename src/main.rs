use crate::options::options::Options;
use crate::options::character_type::CharacterType;
use crate::algorithm::algorithm::GeneratorPtr;
use crate::options::password_length::PasswordLength;

mod algorithm;
mod options;

fn main() {
    let options = Options::from_args();
    let mut algorithm = algorithm::algorithm::Algorithm::new(options.chars.len());

    for character_type in &options.chars {
        let generator = match character_type {
            CharacterType::LowerCase => Box::new(algorithm::generator_lowercase::GeneratorLowerCase) as GeneratorPtr,
            CharacterType::UpperCase => Box::new(algorithm::generator_uppercase::GeneratorUpperCase) as GeneratorPtr,
            CharacterType::Numbers => Box::new(algorithm::generator_number::GeneratorNumber) as GeneratorPtr,
            CharacterType::SpecialCharacters => Box::new(algorithm::generator_symbol::GeneratorSymbol) as GeneratorPtr,
        };

        algorithm.add_generator(generator);
    }

    if options.tokens > 1 {
        let tokenizer = Box::new(algorithm::generator_tokenizer::GeneratorTokenizer::new(options.tokens));
        algorithm.add_generator(tokenizer);
    }

    let average_string_length = match options.length {
        PasswordLength::Short => 16,
        PasswordLength::Regular => 24,
        PasswordLength::Long => 32,
    };

    for _ in 0..options.num {
        let password = algorithm.generate(average_string_length);
        println!("{}", password);
    }
}
