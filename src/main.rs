use crate::options::options::Options;

mod algorithm;
mod options;

fn main() {
    let options = Options::from_args();

    for _ in 0..options.num {
        let password = if options.tokens == 0 {
            algorithm::simple::generate_password(&options.length, &options.chars)
        } else {
            algorithm::tokenized::generate(options.tokens, &options.length, &options.chars)
        };

        println!("{}", password);
    }
}
