use rand::{seq, Rng};

pub struct Pattern<'a>(&'a str);

impl<'a> Pattern<'a> {
    const fn from_str(s: &'a str) -> Self {
        Self(s)
    }

    fn choose_random_char<R: Rng>(&self, rng: &mut R) -> Option<char> {
        seq::IteratorRandom::choose(self.0.chars(), rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choose_random_char() {
        let pattern = Pattern::from_str("abc");
        let mut rng = rand::rng();

        let char = pattern.choose_random_char(&mut rng).unwrap();

        assert!(pattern.0.contains(char));
    }
}
