use crate::options::options::Options;

mod algorithm;
mod options;

fn main() {
    let options = Options::from_args();
    let password = algorithm::simple::generate_password(options.length, &options.chars);
    println!("{}", password);
}
