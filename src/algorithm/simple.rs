use crate::options::password_length::PasswordLength;
use rand::Rng;
use crate::algorithm::generator::generate_random_lowercase_letter;

const LENGTH_BOUNDS: usize = 2;

pub fn generate_password(average_length: usize) -> String {
    let mut thread_rng = rand::thread_rng();
    let min_length = average_length - LENGTH_BOUNDS;
    let max_length = average_length + LENGTH_BOUNDS;
    let password_length = thread_rng.gen_range(min_length, max_length);
    let mut password = String::with_capacity(password_length);

    for _ in 0..password_length {
        password.push(generate_random_lowercase_letter(&mut thread_rng));
    }

    password
}
