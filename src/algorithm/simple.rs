use rand::prelude::ThreadRng;
use crate::options::character_type::CharacterType;
use crate::options::password_length::PasswordLength;
use crate::algorithm::generator::{generate_random_lowercase_letter, calculate_string_length, generate_random_uppercase_letter, generate_random_number_letter, generate_random_special_character};
use rand::Rng;

pub fn generate_simple(password_length: &PasswordLength, character_types: &Vec<CharacterType>) -> String {
    let average_length = to_average_length(password_length);
    let mut thread_rng = rand::thread_rng();
    let mut password_length = calculate_string_length(average_length, &mut thread_rng);
    let mut password = String::with_capacity(password_length);

    for character_type in character_types {
        let generated_password = match character_type {
            CharacterType::LowerCase => generate(password_length, || generate_random_lowercase_letter(&mut thread_rng)),
            CharacterType::UpperCase => generate(password_length, || generate_random_uppercase_letter(&mut thread_rng)),
            CharacterType::Numbers => generate(password_length, || generate_random_number_letter(&mut thread_rng)),
            CharacterType::SpecialCharacters => generate(password_length, || generate_random_special_character(&mut thread_rng)),
        };

        password = combine(password, generated_password, &mut thread_rng);
        password_length /= 2;
    }

    password
}

fn to_average_length(password_length: &PasswordLength) -> usize {
    match password_length {
        PasswordLength::Short => 16,
        PasswordLength::Regular => 24,
        PasswordLength::Long => 32,
    }
}

fn generate(length: usize, mut generator: impl FnMut() -> char) -> String {
    let mut password = String::with_capacity(length);

    for _ in 0..length {
        password.push(generator());
    }

    password
}

fn combine(password: String, template: String, thread_rng: &mut ThreadRng) -> String {
    if password.is_empty() {
        template
    } else {
        let mut characters = password.into_bytes();

        template.chars().for_each(|character| {
            let target_index = thread_rng.gen_range(0, characters.len());
            characters[target_index] = character as u8;
        });

        String::from_utf8(characters).unwrap()
    }
}
