#[derive(Debug)]
pub struct Token {
    value: String,
}

impl Token {
    pub fn new(capacity: usize) -> Self {
        Self {
            value: String::with_capacity(capacity)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::password::token::Token;
    use spectral::prelude::*;

    const CAPACITY: usize = 10;

    #[test]
    fn should_generate_default_empty_token() {
        let token = Token::new(CAPACITY);

        assert_that(&token.value.len()).is_equal_to(0);
        assert_that(&token.value.capacity()).is_equal_to(CAPACITY);
    }
}
