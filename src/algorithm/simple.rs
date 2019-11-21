use rand::Rng;
use crate::options::password_length::PasswordLength;
use crate::options::character_type::CharacterType;
use crate::algorithm::generator::{generate_random_lowercase_letter, generate_base_password};

const PASSWORD_AVERAGE_LENGTH_SHORT: usize = 8;
const PASSWORD_AVERAGE_LENGTH_REGULAR: usize = 12;
const PASSWORD_AVERAGE_LENGTH_LONG: usize = 16;

pub fn generate_password(average_length: usize, character_types: &Vec<CharacterType>) -> String {
    let mut thread_rng = rand::thread_rng();
    let mut password = generate_base_password(average_length, &mut thread_rng, || {
        generate_random_lowercase_letter(&mut thread_rng)
    });

    // 2 - 60/40
    // 3 - 60/40/20
    // 4 - 60/40/20/10

    for _ in 0..password_length {
        password.push(generate_random_lowercase_letter(&mut thread_rng));
    }

    password
}

fn generate_base(length: usize, character_type: CharacterType) -> String {
    match character_type {
        CharacterType::LowerCase => generate(length, || {
            generate_random_lowercase_letter()
        }),
        CharacterType::UpperCase => {}
        CharacterType::Numbers => {}
        CharacterType::SpecialCharacters => {}
    }
}

fn generate(length: usize, generator: fn() -> char) -> String {
    let mut password = String::with_capacity(length);
    for _ in 0..length {
        password.push(generator());
    }

    password
}
