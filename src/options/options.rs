use structopt::StructOpt;
use crate::options::character_type::CharacterType;
use crate::options::password_length::PasswordLength;

#[derive(StructOpt, Debug)]
#[structopt(about = "Password generation tool")]
pub struct Options {
    #[structopt(short, long, possible_values = &CharacterType::variants(),
    case_insensitive = true, default_value = "l,u,n", use_delimiter = true, help = "Character types")]
    pub chars: Vec<CharacterType>,
    #[structopt(short, long, possible_values = &PasswordLength::variants(),
    case_insensitive = true, default_value = "r", help = "Password length")]
    pub length: PasswordLength,
    #[structopt(short, long, default_value = "1", help = "Number of passwords to generate")]
    pub num: u8,
    #[structopt(short, long, default_value = "3", help = "Number of password tokens")]
    pub tokens: usize
}

impl Options {
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }
}
