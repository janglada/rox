#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma, Dot, Minus, Plus,
    SemiColon, Slash, Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
   // pub lexeme: String,
    pub line: isize,

    pub start: usize,
    pub len: usize,
}







impl Token {
    pub fn new(token_type:TokenType, start: usize, len: usize,    line: isize)-> Self {
        Token {
            token_type, start, len,  line
        }
    }
}