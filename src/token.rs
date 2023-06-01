#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub enum TokenType {
    // Special characters
    NewLine,    // "\n"
    Semicolon,  // ";"
    ReturnType, // "->"
    LParen,     // "("
    RParen,     // ")"

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,

    // Blocks
    Do,         // "do"
    Then,       // "then"
    End,        // "end"

    // Other Keywords
    Function,   // "fn'"

    // Special tokens
    Ident(String),      // Identifier
    Number(String, u8), // Number tuple: (number, base)
    String,             // String enclosed in '"' (contents)
    Charstring,         // String enclosed in "'"
    Illegal(String),    // Illegal (reason)
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub ident: String,
    pub row: u128,
    pub col: u128,
}

#[allow(dead_code)]
impl Token {
    pub fn new(t_type: TokenType, ident: String, row: u128, col: u128) -> Self {
        Token {
            t_type,
            ident,
            row,
            col,
        }
    }
}

/*

fn main do
    print "Hello, World!"
end

*/