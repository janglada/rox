use std::error::Error;
use crate::token::Token;

pub struct  Parser<'a> {
    pub current: Token,
    pub previous: Token,
    pub result: Result<(), &'a str>,
    pub panic_mode: bool
}

impl<'a> Parser<'a> {
   pub  fn new() -> Self {
        Parser {
            current: Token::dummy(),
            previous: Token::dummy(),
            result: Ok(()),
            panic_mode:false
        }
    }
}
