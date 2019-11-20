use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "A password generation tool", author = "Artem Korobko")]
pub struct Options {
}

impl Options {
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }
}
