use crate::options::options::Options;

mod algorithm;
mod options;

fn main() {
    let options = Options::from_args();

    for _ in 0..options.num {
        let password = if options.tokens == 0 {
            algorithm::simple::generate_simple(&options.length, &options.chars)
        } else {
            algorithm::tokenized::generate_tokenized(options.tokens, &options.length, &options.chars)
        };

        println!("{}", password);
    }
}
