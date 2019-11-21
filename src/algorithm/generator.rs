use rand::Rng;
use rand::prelude::ThreadRng;

const AVERAGE_LENGTH_BOUNDS: usize = 2;
const SPECIAL_CHARACTERS_TABLE: [char; 29] = [
    '!', '#', '$', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';',
    '<', '=', '>', '?', '@', '[', ']', '^', '_', '`', '{', '|', '}', '~'
];

pub fn generate_base_password(average_length: usize, thread_rng: &mut ThreadRng, generator: fn() -> char) -> String {
//    let min_length = average_length - LENGTH_BOUNDS;
//    let max_length = average_length + LENGTH_BOUNDS;
//    let password_length = thread_rng.gen_range(min_length, max_length);
//    String::with_capacity(password_length)
    "123".into()
}

pub fn generate_random_lowercase_letter(thread_rng: &mut ThreadRng) -> char {
    thread_rng.gen_range::<u8, u8, u8>(97, 123) as char
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
