use super::ast::*;

pub struct Tokenizer<'s> {
    source:     &'s [u8],
    ch_pos:     usize,              // Current position of the lexer
    source_pos: SourcePos,          // col & row

    tokens:     Vec<Token<'s>>      // The vector of tokens
}

impl<'s> Tokenizer<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self {
            source,
            ch_pos: 0,
            source_pos: SourcePos { line: 0, column: 0 },
            tokens: vec![],
        }
    }

    // pub fn tokenize(source: &'s [u8]) -> ParseResult<Vec<Token<'s>>> {
    //     let mut tokenizer = Self::new(source);
    //     while let Some(t) = tokenizer.next_token() {

    //     }
    //     tokenizer.run()?;
    //     Ok(tokenizer.tokens)
    // }

    pub fn tokenize(source: &'s [u8]) -> ParseResult<Vec<Token<'s>>> {
        let mut toker = Self::new(source);
        toker.run()?;
        Ok(toker.tokens)
    }

    pub fn run(&mut self) -> ParseResult<()> {
        while let Some(t) = self.next_token()? {
            self.tokens.push(t);
        }
        Ok(())
    }

    #[inline(always)]
    fn pos(&self) -> SourcePos {
        SourcePos { line: self.source_pos.line, column: self.source_pos.column }
    }

    #[inline(always)]
    fn peek_ch(&self, offset: usize) -> Option<u8> {
        self.source.get(self.ch_pos + offset).cloned()
    }

    #[inline(always)]
    fn peek_ch_or_0(&self, offset: usize) -> u8 {
        *self.source.get(self.ch_pos + offset).unwrap_or(&0)
    }

    #[inline(always)]
    fn consume_ch(&mut self, count: usize) {
        self.ch_pos += count
    }

    #[inline(always)]
    fn consume_ch_while<F: Fn(u8) -> bool>(&mut self, w: F) {
        while w(self.peek_ch_or_0(0)) {
            self.consume_ch(1)
        }
    }

    #[inline(always)]
    fn consume_ch_before_eof_while<F: Fn(u8) -> bool>(&mut self, w: F) -> Result<(), ()> {
        while w(match self.peek_ch(0) {
            Some(i) => i,
            None => return Err(())
        }) {
            self.consume_ch(1)
        }
        Ok(())
    }

    // Creates a new token that ends on current ch_pos
    #[inline(always)]
    fn new_token(&self, begin: SourcePos, data: TokenData<'s>) -> Token<'s> {
        Token::new(begin, self.pos(), data)
    }

    // Skip all the junk
    fn skip_whitespace(&mut self) {
        while let Some(current) = self.peek_ch(0) {
            match current as char {
                // New lines
                '\n' => {
                    self.consume_ch(1);
                    self.source_pos.line += 1;
                    self.source_pos.column = 0;
                },
                // Useless junk
                ' ' | '\r' | '\t' => {
                    self.consume_ch(1);
                },
                // Comments
                '-' => {
                    // If not a comment, break
                    if self.peek_ch_or_0(1) != '-' as u8 {
                        break
                    }

                    if self.peek_ch_or_0(2) == '-' as u8 {
                        // Multiline comment
                        self.consume_ch(3);
                        'outer: loop {
                            for i in 0..3 {
                                match self.peek_ch(i) {
                                    Some(c) => if c != '-' as u8 { continue 'outer },
                                    None => break 'outer
                                }
                            }
                            self.consume_ch(1)
                        }
                    }
                    else {
                        // One-line comment
                        self.consume_ch_while(|c| c != '\n' as u8)
                    }
                },
                _ => break
            }
        }
    }

    // Get next token
    fn next_token(&mut self) -> ParseResult<Option<Token<'s>>> {
        self.skip_whitespace();

        let Some(current_ch) = self.peek_ch(0) else {
            return Ok(None)
        };

        let begin_ch_pos = self.ch_pos;
        let begin_source_pos = self.pos();

        macro_rules! tok_ch {
            ($data: expr) => {{
                self.consume_ch(1);
                return Ok(Some(self.new_token(begin_source_pos, $data)));
            }};
        }

        // Return a token consisting of 2 characters if possible, otherwise return the single character one.
        macro_rules! tok_ch_peek_2 {
            ($data_single: expr, $match_array: expr) => {{
                self.consume_ch(1);

                // 2-char token
                for (fchar, fdata) in $match_array {
                    if fchar as u8 == self.peek_ch_or_0(0) {
                        self.consume_ch(1);
                        return Ok(Some(self.new_token(begin_source_pos, fdata)))
                    }
                }

                // single-char token
                return Ok(Some(self.new_token(begin_source_pos, $data_single)))
            }}
        }

        match current_ch as char {
            '(' => tok_ch!(TokenData::LParen),
            ')' => tok_ch!(TokenData::RParen),
            '[' => tok_ch!(TokenData::LBracket),
            ']' => tok_ch!(TokenData::RBracket),
            '{' => tok_ch!(TokenData::LCurly),
            '}' => tok_ch!(TokenData::RCruly),

            '.' => tok_ch!(TokenData::Dot),
            ',' => tok_ch!(TokenData::Comma),
            ':' => tok_ch!(TokenData::Colon),
            ';' => tok_ch!(TokenData::Semicolon),

            '+' => tok_ch_peek_2!(TokenData::OpAdd,     [('=', TokenData::OpAddAssign)]),
            '-' => tok_ch_peek_2!(TokenData::OpMinus,   [('=', TokenData::OpMinusAssign)]),
            '*' => tok_ch_peek_2!(TokenData::OpMul,     [('=', TokenData::OpMulAssign)]),
            '/' => tok_ch_peek_2!(TokenData::OpDiv,     [('=', TokenData::OpDivAssign)]),

            '=' => tok_ch_peek_2!(TokenData::OpAssign, [
                ('=', TokenData::OpEqual),
                ('>', TokenData::RFatArrow),
            ]),

            '<' => tok_ch_peek_2!(TokenData::OpLessThan,    [('=', TokenData::OpLessEqThan)]),
            '>' => tok_ch_peek_2!(TokenData::OpGreaterThan, [('=', TokenData::OpGreaterEqThan)]),

            '!' => tok_ch_peek_2!(TokenData::OpNot,         [('=', TokenData::OpNotEqual)]),

            // String
            '\'' => {
                self.consume_ch(1);
                match self.consume_ch_before_eof_while(|c| c != '\'' as u8) {
                    Ok(()) => (),
                    Err(()) => return Err(ParseError {
                        source_view: SourceView { start: begin_source_pos, end: self.pos() },
                        data: ParseErrorData::TrailingInput
                    })
                }

                self.consume_ch(1);

                return Ok(Some(
                    self.new_token(begin_source_pos, TokenData::String(
                        unsafe {
                            core::str::from_utf8_unchecked(&self.source[begin_ch_pos..self.ch_pos])
                        }
                    ))
                ))
            },
            _ => (),
        }

        // Keyword or ident
        if current_ch.is_ascii_alphabetic() || current_ch == '_' as u8 {
            self.consume_ch(1);
            self.consume_ch_while(|c| c.is_ascii_alphabetic() || c == '_' as u8);
            let word = unsafe {
                core::str::from_utf8_unchecked(
                    &self.source[begin_ch_pos..self.ch_pos]
                )
            };

            return Ok(Some(self.new_token(begin_source_pos, match word {
                "do"                => TokenData::KwDo,
                "end"               => TokenData::KwEnd,
                "local"             => TokenData::KwStatic,
                "let"               => TokenData::KwLet,
                "static"            => TokenData::KwConst,
                "if"                => TokenData::KwIf,
                "elif"              => TokenData::KwElif,
                "else"              => TokenData::KwElse,
                "fn"                => TokenData::KwFn,
                "and"               => TokenData::KwAnd,
                "or"                => TokenData::KwOr,
                "false"             => TokenData::Bool(false),
                "true"              => TokenData::Bool(true),

                _ => TokenData::Ident(word)
            })))
        }

        // Number
        if current_ch.is_ascii_digit() {
            // Cover regular '123456789' notation, as well as '0x123456789ABCDEF'
            let base: u8 = match current_ch as char {
                '0' => {
                    self.consume_ch(1);
                    let base = match self.peek_ch_or_0(0) as char {
                        'b' => 2,
                        'o' => 8,
                        'x' => 16,
                        _ => {
                            self.consume_ch(1);
                            return Ok(Some(self.new_token(begin_source_pos, TokenData::Number(10, "0"))))
                        }
                    };
                    self.consume_ch(1);
                    
                    if !self.peek_ch_or_0(0).is_ascii_digit() {
                        return Err(ParseError {
                            source_view: SourceView {
                                start: begin_source_pos,
                                end: self.pos()
                            },
                            data: ParseErrorData::UnexpectedChar,
                        })
                    } 
                    
                    base
                }
                _ => 10,
            };

            // TODO: Add non-base10 support
            self.consume_ch_while(|c| c.is_ascii_digit());

            return Ok(Some(self.new_token(begin_source_pos, TokenData::Number(base, unsafe {
                core::str::from_utf8_unchecked(
                    &self.source[begin_ch_pos..self.ch_pos]
                )
            }))))
        }

        Err(ParseError {
            source_view: SourceView {
                start: SourcePos { line: 0, column: 0 },
                end: SourcePos { line: 0, column: 0 }
            },
            data: ParseErrorData::Unreachable("next_token()")
        })
    }
}