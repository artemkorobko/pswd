use rand::Rng;
use rand::prelude::ThreadRng;
use std::fmt::Debug;

pub trait Generator: Debug {
    fn generate(&self, sequence_number: usize, initial_value: String, random: &mut ThreadRng) -> String;
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
