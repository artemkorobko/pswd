pub trait Generator {
    fn generate<R: rand::Rng>(&self, buf: &mut [u8], rng: &mut R);
    fn chain<T>(self, gen: T) -> Chained<Self, T>
    where
        T: Generator,
        Self: Sized,
    {
        Chained::new(self, gen)
    }
}

pub struct Chained<P, N> {
    prev: P,
    next: N,
}

impl<P, N> Chained<P, N> {
    pub fn new(prev: P, next: N) -> Self {
        Self { prev, next }
    }
}

impl<P: Generator, N: Generator> Generator for Chained<P, N> {
    fn generate<R: rand::Rng>(&self, buf: &mut [u8], rng: &mut R) {
        self.prev.generate(buf, rng);
        self.next.generate(buf, rng);
    }
}

#[derive(Default)]
pub struct Baseline;

impl Generator for Baseline {
    fn generate<R: rand::Rng>(&self, buf: &mut [u8], rng: &mut R) {
        let vowels = [b'a', b'e', b'i', b'o', b'u'];
        let consonants = [
            b'b', b'c', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b'm', b'n', b'p', b'q', b'r',
            b's', b't', b'v', b'w',
        ];
        let mut vowel = rng.gen_bool(0.5);
        for symbol in buf {
            if *symbol != b'-' {
                if vowel {
                    *symbol = vowels[rng.gen_range(0..vowels.len())];
                    vowel = rng.gen_bool(0.2);
                } else {
                    *symbol = consonants[rng.gen_range(0..consonants.len())];
                    vowel = true;
                }
            }
        }
    }
}

pub struct UpperCaseLetter {
    probability: f32,
}

impl UpperCaseLetter {
    pub fn new(probability: f32) -> Self {
        Self { probability }
    }
}

impl Generator for UpperCaseLetter {
    fn generate<R: rand::Rng>(&self, buf: &mut [u8], rng: &mut R) {
        let mut retries = 0;
        let mut cnt = symbols_count(buf.len(), self.probability);
        while cnt > 0 || retries >= buf.len() {
            let pos = rng.gen_range(0..buf.len());
            if buf[pos].is_ascii_lowercase() {
                buf[pos] = buf[pos].to_ascii_uppercase();
                cnt -= 1;
            } else {
                retries += 1;
            }
        }
    }
}

pub struct Digit {
    probability: f32,
}

impl Digit {
    pub fn new(probability: f32) -> Self {
        Self { probability }
    }
}

impl Generator for Digit {
    fn generate<R: rand::Rng>(&self, buf: &mut [u8], rng: &mut R) {
        let mut retries = 0;
        let mut cnt = symbols_count(buf.len(), self.probability);
        while cnt > 0 || retries >= buf.len() {
            let pos = rng.gen_range(0..buf.len());
            let symbol = buf[pos];
            if symbol.is_ascii_lowercase() || symbol.is_ascii_uppercase() {
                buf[pos] = rng.gen_range(b'0'..=b'9');
                cnt -= 1;
            } else {
                retries += 1;
            }
        }
    }
}

fn symbols_count(len: usize, probability: f32) -> usize {
    let cnt = len as f32 * probability;
    std::cmp::max(1, cnt.ceil() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_generators() {
        let mut buf = vec![0u8; 10];
        let baseline = Baseline::default();
        let upper_case = UpperCaseLetter::new(0.2);
        let digit = Digit::new(0.1);

        baseline
            .chain(upper_case)
            .chain(digit)
            .generate(&mut buf, &mut rand::thread_rng());

        let mut lower_case_letters = 0;
        let mut upper_case_letters = 0;
        let mut digits = 0;

        for symbol in buf {
            if symbol.is_ascii_lowercase() {
                lower_case_letters += 1;
            } else if symbol.is_ascii_uppercase() {
                upper_case_letters += 1;
            } else if symbol.is_ascii_digit() {
                digits += 1;
            }
        }

        assert!(lower_case_letters > upper_case_letters);
        assert!(upper_case_letters > digits);
    }
}
