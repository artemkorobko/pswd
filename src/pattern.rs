use rand::{seq, Rng};
use std::collections::HashSet;

pub struct Pattern<'a>(&'a str);

impl<'a> Pattern<'a> {
    const fn from_str(s: &'a str) -> Self {
        Self(s)
    }

    pub fn choose_random_char<R: Rng>(&self, rng: &mut R) -> Option<char> {
        seq::IteratorRandom::choose(self.0.chars(), rng)
    }

    pub fn choose_random_char_except<R: Rng>(
        &self,
        rng: &mut R,
        exception: char,
        mut retries: usize,
    ) -> Option<char> {
        if !self.contains_only_uniques() {
            return None;
        }

        while retries > 0 {
            let chosen = self.choose_random_char(rng);
            if chosen? != exception {
                return chosen;
            }

            retries -= 1;
        }

        None
    }

    fn contains_only_uniques(&self) -> bool {
        let mut symbols = HashSet::with_capacity(self.0.len());

        for symbol in self.0.chars() {
            if !symbols.insert(symbol) {
                return false;
            }
        }

        true
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

    #[test]
    fn do_not_choose_random_char_if_pattern_is_empty() {
        let pattern = Pattern::from_str("");
        let mut rng = rand::rng();

        let char = pattern.choose_random_char(&mut rng);

        assert!(char.is_none());
    }

    #[test]
    fn choose_random_char_except() {
        let pattern = Pattern::from_str("abc");
        let mut rng = rand::rng();

        for _ in 0..10 {
            let char = pattern.choose_random_char_except(&mut rng, 'a', 10).unwrap();
            assert_ne!(char, 'a');
            assert!(pattern.0.contains(char));
        }
    }

    #[test]
    fn do_not_choose_random_char_except_if_pattern_is_not_unique() {
        let pattern = Pattern::from_str("aabbcc");
        let mut rng = rand::rng();

        let char = pattern.choose_random_char_except(&mut rng, 'a', 3);

        assert!(char.is_none());
    }
}
