use crate::options::options::Options;
use crate::options::password_length::PasswordLength;

mod algorithm;
mod options;

const PASSWORD_AVERAGE_LENGTH_SHORT: usize = 8;
const PASSWORD_AVERAGE_LENGTH_REGULAR: usize = 12;
const PASSWORD_AVERAGE_LENGTH_LONG: usize = 16;

fn main() {
    let options= Options::from_args();
    println!("{:?}", options.chars);

    let password = match options.length {
        PasswordLength::Short => generate_password(PASSWORD_AVERAGE_LENGTH_SHORT),
        PasswordLength::Regular => generate_password(PASSWORD_AVERAGE_LENGTH_REGULAR),
        PasswordLength::Long => generate_password(PASSWORD_AVERAGE_LENGTH_LONG),
    };

    println!("{}", password);
}

fn generate_password(average_length: usize) -> String {
    algorithm::simple::generate_password(average_length)
}
