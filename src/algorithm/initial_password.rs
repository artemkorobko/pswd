use rand::Rng;
use rand::rngs::ThreadRng;

const AVERAGE_LENGTH_BOUNDS: usize = 2;

pub fn build_initial_password(average_password_length: usize, mut random: &mut ThreadRng) -> String {
    let string_length = calculate_string_length(average_password_length, &mut random);
    String::with_capacity(string_length)
}

fn calculate_string_length(average_length: usize, random: &mut ThreadRng) -> usize {
    let min_length = average_length - AVERAGE_LENGTH_BOUNDS;
    let max_length = average_length + AVERAGE_LENGTH_BOUNDS;
    random.gen_range(min_length, max_length)
}

#[cfg(test)]
mod tests {
    use crate::algorithm::initial_password::*;

    #[test]
    fn should_build_initial_password() {
        let average_string_length = 10;
        let mut random = rand::thread_rng();

        let result = build_initial_password(average_string_length, &mut random);

        assert!(result.capacity() >= average_string_length - AVERAGE_LENGTH_BOUNDS);
        assert!(result.capacity() <= average_string_length + AVERAGE_LENGTH_BOUNDS);
    }
}
