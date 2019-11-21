use crate::options::options::Options;
use crate::options::password_length::PasswordLength;
use crate::options::character_type::CharacterType;

mod algorithm;
mod options;

fn main() {
    let options = Options::from_args();
//    let password = generate_password(average_password_length);
//    println!("{}", password);
}

//fn generate_password(average_length: usize) -> String {
//    algorithm::simple::generate_password(average_length)
//}
