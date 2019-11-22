use rand::Rng;
use rand::prelude::ThreadRng;
use std::fmt::Debug;

pub trait Generator: Debug {
    fn generate(&self, sequence_number: usize, initial_value: String, random: &mut ThreadRng) -> String {
        if sequence_number == 0 {
            self.generate_string(initial_value.capacity(), random)
        } else {
            let expected_length = initial_value.capacity() / (sequence_number + 1);
            let result = self.generate_string(expected_length, random);
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

pub fn calculate_string_length(average_length: usize, thread_rng: &mut ThreadRng) -> usize {
    let min_length = average_length - AVERAGE_LENGTH_BOUNDS;
    let max_length = average_length + AVERAGE_LENGTH_BOUNDS;
    thread_rng.gen_range(min_length, max_length)
}

fn combine_strings(left: String, right: String, random: &mut ThreadRng) -> String {
    let mut characters = left.into_bytes();

    right.chars().for_each(|character| {
        let target_index = random.gen_range(0, characters.len());
        characters[target_index] = character as u8;
    });

    String::from_utf8(characters).unwrap()
}
