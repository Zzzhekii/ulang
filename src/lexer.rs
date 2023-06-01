use crate::token;

const SPECIAL_CHARACTERS: &'static [char] = &['\n', ';', '.', ',', '(', ')', '+', '-', '*', '/', '#'];
const WHITESPACES: &'static [char] = &[' ', '\t', '\r'];
const DIGITS: &'static [char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

#[allow(dead_code)]
pub struct Lexer {
    // Source
    source:         String,

    // Tokens
    peek_t:         Option<token::Token>,       // (buffer) Returned by peek() & next(), with second also updating this field

    // Characters
    current_char:   Option<char>,               // Current character
    peek_char:      Option<char>,               // Next character

    // Positional metadata
    col:            u128,
    row:            u128,
    pos:            usize,                      // Position in the source
}

impl Lexer {
    pub fn from_str(string: &str) -> Self {
        let mut lexer = Self {
            source:         String::from(string),
            peek_t:         None,
            current_char:   None,
            peek_char:      None,
            col:            0,
            row:            0,
            pos:            0,
        };

        // Fill the peek_t
        lexer.advance();
        lexer.advance();

        return lexer
    }

    // Return the next token & and advance the lexer
    pub fn next(&mut self) -> Option<token::Token> {
        let tmp = self.peek_t.clone();
        self.advance();
        return tmp
    }

    // Return the next token without advancing the lexer (peek)
    pub fn peek(&self) -> Option<token::Token> {
        self.peek_t.clone()
    }

    // Advance the lexer (set peek_t. if possible + other side effects (good luck lol, you imbecile (adressed to me btw)))
    fn advance(&mut self) {
        self.skip_whitespaces();

        let (row, col) = (self.row + 1, self.col);

        if let Some(c) = self.next_character() {
            self.peek_t = Some(
                match c {
                    // Single-character tokens
                    '\n' => token::Token::new(
                        token::TokenType::NewLine,
                        String::from(c),
                        row, col
                    ),
                    ';' => token::Token::new(
                        token::TokenType::Semicolon,
                        String::from(c),
                        row, col
                    ),
                    '(' => token::Token::new(
                        token::TokenType::LParen,
                        String::from(c),
                        row, col
                    ),
                    ')' => token::Token::new(
                        token::TokenType::RParen,
                        String::from(c),
                        row, col
                    ),
                    '+' => token::Token::new(
                        token::TokenType::Plus,
                        String::from(c),
                        row, col
                    ),
                    '-' => token::Token::new(
                        token::TokenType::Minus,
                        String::from(c),
                        row, col
                    ),
                    '*' => token::Token::new(
                        token::TokenType::Multiply,
                        String::from(c),
                        row, col
                    ),
                    '/' => token::Token::new(
                        token::TokenType::Divide,
                        String::from(c),
                        row, col
                    ),

                    // Number
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        let start_pos = self.pos;
                        let (number, base) = self.scan_number().unwrap();
                        token::Token::new(
                            token::TokenType::Number(number, base),
                            self.source[start_pos-2 .. self.pos-1].to_string(),
                            row, col
                        )
                    }

                    // String
                    '\'' => {
                        token::Token::new(
                            token::TokenType::String,
                            String::from(self.scan_string('\'').unwrap()),
                            row, col
                        )
                    },
                    
                    // Keywords & idents
                    _ => {
                        let ident = self.scan_ident();

                        let mut t = token::Token::new(
                            token::TokenType::Illegal("undefined".to_string()),
                            ident.clone(),
                            row, col
                        );

                        match ident.clone().as_str() {
                            "do" => t.t_type = token::TokenType::Do,
                            "end" => t.t_type = token::TokenType::End,
                            "then" => t.t_type = token::TokenType::Then,

                            "fn" => t.t_type = token::TokenType::Function,

                            _ => t.t_type = token::TokenType::Ident(ident),
                        }

                        t
                    } 
                }
            );
        } else {
            self.peek_t = None
        }
    }

    // Call advance_character() & return the current_char
    fn next_character(&mut self) -> Option<char> {
        self.advance_character();
        self.current_char
    }

    // Advance position & set current_char with peek_char
    fn advance_character(&mut self) {
        // Update current_char & peek_char
        self.current_char = self.peek_char;
        self.peek_char = self.source.chars().nth(self.pos);

        // Update the positional metadata
        if let Some(c) = self.peek_char {
            self.col += 1;

            // New line
            if c == '\n' {
                self.col = 0;
                self.row += 1;
            }

            self.pos += 1;
        }
    }

    // Skip all the useless special characters & whitespaces
    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.peek_char {
            if WHITESPACES.contains(&c) {
                self.advance_character()
            } else {
                break
            }
        }
    }

    // Scan for an ident (untill a whitespace or smth like that).
    fn scan_ident(&mut self) -> String {
        let start_pos = self.pos;

        while let Some(c) = self.peek_char {
            if WHITESPACES.contains(&c) || SPECIAL_CHARACTERS.contains(&c) {
                return self.source[start_pos-2 .. self.pos-1].to_string()
            }

            self.advance_character()
        }

        self.source[start_pos-2 .. self.pos].to_string()
    }

    // Scan for a string untill the ending character, or return an error if ends.
    fn scan_string(&mut self, ending_char: char) -> Result<String, String> {
        let start_pos = self.pos;

        while let Some(c) = self.peek_char {
            self.advance_character();
            if c == ending_char {
                return Ok(self.source[start_pos-1 .. self.pos-2].to_string())
            }
        }
        
        Err(format!("Expected \"{}\", got EOF", ending_char))
    }

    // Scan for a number (123 or 0x123, 0b123, etc. notation) and return (number, base), or an error
    fn scan_number(&mut self) -> Result<(String, u8), String> {
        let mut base: u8 = 10;
        
        if self.current_char == Some('0') {
            // 0x123-like notation

            base = match self.peek_char.unwrap() {
                'b' => 2,
                'o' => 8,
                'x' => 16,
                _ => return Err(format!("Expected a valid base prefix, got: \"{}\"", self.peek_char.unwrap())),
            };

            self.advance_character();
            self.advance_character();   
        }

        let number = self.scan_ident();
        if !isNumber(&number) {
            return Err(format!("Expected a number, got: \"{}\"", number))
        }

        return Ok((number, base))
    }
}

fn isNumber(string: &str) -> bool {
    for c in string.chars().into_iter() {
        if !DIGITS.contains(&c) {
            return false
        }
    }

    true
}