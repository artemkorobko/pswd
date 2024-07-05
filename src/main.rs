use clap::Parser;
use generator::Generator;

mod generator;
mod options;

fn main() {
    let args = options::Args::parse();
    let mut rng = rand::thread_rng();
    let mut buf = create_password_buffer(args.length.into(), args.tokens);
    build_generators_chain().generate(&mut buf, &mut rng);
    let password = String::from_utf8(buf).expect("Can't to convert generated password to string");
    println!("{password}");
}

fn create_password_buffer(len: usize, tokens: usize) -> Vec<u8> {
    let max_len = (len * tokens) + 2;
    let mut password = vec![0u8; max_len];
    for pos in 1..tokens {
        password[(pos * (len + 1)) - 1] = b'-';
    }
    password
}

fn build_generators_chain() -> impl Generator {
    let upper_case_generator = generator::UpperCaseLetter::new(0.1);
    let digit_generator = generator::Digit::new(0.05);
    generator::Baseline
        .chain(upper_case_generator)
        .chain(digit_generator)
}
