use crate::options::Options;

mod generator;
mod options;

fn main() {
    Options::from_args();
}
