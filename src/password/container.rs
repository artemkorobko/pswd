use crate::password::token::Token;

#[derive(Debug)]
pub struct Container {
    tokens: Vec<Token>,
}
