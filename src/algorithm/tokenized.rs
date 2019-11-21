use crate::options::password_length::PasswordLength;
use crate::options::character_type::CharacterType;
use crate::algorithm::simple;
use itertools::Itertools;

pub fn generate(tokens: u8, password_length: &PasswordLength, character_types: &Vec<CharacterType>) -> String {
    let bytes = simple::generate_password(password_length, character_types).into_bytes();
    bytes.chunks(bytes.len() / tokens as usize)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .join("-")
}
