use generator::Generator;
use structopt::StructOpt;

mod generator;
mod options;

fn main() {
    let args = options::Args::from_args();
    let mut rng = rand::thread_rng();
    let mut buf = create_password_buffer(args.length.into(), args.tokens);
    build_generators_chain().generate(&mut buf, &mut rng);
    let password = String::from_utf8(buf).expect("Can't to convert generated password to string");
    println!("{password}");
    if args.clipboard {
        use clipboard::{ClipboardContext, ClipboardProvider};
        ClipboardContext::new()
            .expect("Can't create clipboard context")
            .set_contents(password)
            .expect("Can't copy generated password to clipboard");
        println!("Copied to clipboard.");
    }
}

fn create_password_buffer(len: usize, tokens: usize) -> Vec<u8> {
    let total_len = (len * tokens) + 2;
    let mut password = vec![0u8; total_len];
    for pos in 1..tokens {
        password[(pos * (len + 1)) - 1] = b'-';
    }
    password
}

fn build_generators_chain() -> impl Generator {
    let baseline = generator::Baseline::default();
    let upper_case = generator::UpperCaseLetter::new(0.2);
    let digit = generator::Digit::new(0.05);
    baseline.and_then(upper_case).and_then(digit)
}
