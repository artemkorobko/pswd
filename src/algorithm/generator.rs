use rand::Rng;
use rand::prelude::ThreadRng;
use std::fmt::Debug;

pub trait Generator: Debug {
    fn generate(&self, sequence_number: usize, initial_value: String, random: &mut ThreadRng) -> String {
        if sequence_number == 0 {
            self.generate_string(initial_value.capacity(), random)
        } else {
            let expected_length = initial_value.capacity() / (sequence_number + 1);
            let result = self.generate_string(initial_value.capacity(), random);
            combine_strings(initial_value, result, random)
        }
    }

    fn generate_string(&self, length: usize, random: &mut ThreadRng) -> String {
        let mut result = String::with_capacity(length);

        for _ in 0..length {
            let character = self.generate_character(random);
            result.push(character);
        }

        result
    }

    fn generate_character(&self, random: &mut ThreadRng) -> char;
}

const AVERAGE_LENGTH_BOUNDS: usize = 2;
const SPECIAL_CHARACTERS_TABLE: [char; 29] = [
    '!', '#', '$', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';',
    '<', '=', '>', '?', '@', '[', ']', '^', '_', '`', '{', '|', '}', '~'
];

pub fn calculate_string_length(average_length: usize, thread_rng: &mut ThreadRng) -> usize {
    let min_length = average_length - AVERAGE_LENGTH_BOUNDS;
    let max_length = average_length + AVERAGE_LENGTH_BOUNDS;
    thread_rng.gen_range(min_length, max_length)
}

pub fn generate_random_lowercase_letter(random: &mut ThreadRng) -> char {
    random.gen_range::<u8, u8, u8>(97, 123) as char
}

pub fn generate_random_uppercase_letter(thread_rng: &mut ThreadRng) -> char {
    thread_rng.gen_range::<u8, u8, u8>(65, 91) as char
}

pub fn generate_random_number_letter(thread_rng: &mut ThreadRng) -> char {
    thread_rng.gen_range::<u8, u8, u8>(48, 58) as char
}

pub fn generate_random_special_character(thread_rng: &mut ThreadRng) -> char {
    let index = thread_rng.gen_range(0, SPECIAL_CHARACTERS_TABLE.len() - 1);
    SPECIAL_CHARACTERS_TABLE[index]
}

fn combine_strings(left: String, right: String, random: &mut ThreadRng) -> String {
    let mut characters = left.into_bytes();

    right.chars().for_each(|character| {
        let target_index = random.gen_range(0, characters.len());
        characters[target_index] = character as u8;
    });

    String::from_utf8(characters).unwrap()
}
