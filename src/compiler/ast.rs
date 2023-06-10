// A source view

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SourcePos {
    pub line:   u32,
    pub column: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SourceView {
    pub start:  SourcePos,
    pub end:    SourcePos,
}

// Tokens

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub data:           TokenData<'a>,
    pub source_view:    SourceView,
}

impl<'a> Token<'a> {
    pub fn new(start: SourcePos, end: SourcePos, data: TokenData<'a>) -> Self {
        Self { data, source_view: SourceView { start, end } }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenData<'a> {
    Ident   (&'a str),          // (ident)
    Number  (u8, &'a str),      // (number, base)
    Bool    (bool),             // (bool)
    String  (&'a str),          // (string (without quotes))
    LParen,
    RParen,
    LBracket,
    RBracket,
    LCurly,
    RCruly,
    Dot,
    Comma,
    Colon,
    Semicolon,
    RFatArrow,
    LSlimArrow,
    KwLet,
    KwConst,
    KwStatic,
    KwFn,
    KwDo,
    KwEnd,
    KwIf,
    KwElif,
    KwElse,
    KwAnd,
    KwOr,
    OpAdd,
    OpAddAssign,
    OpMinus,
    OpMinusAssign,
    OpMul,
    OpMulAssign,
    OpDiv,
    OpDivAssign,
    OpEqual,
    OpNotEqual,
    OpNot,
    OpLessThan,
    OpGreaterThan,
    OpLessEqThan,
    OpGreaterEqThan,
    OpAssign,
}

// Compile errors

pub type ParseResult<'a, T> = Result<T, ParseError>;

#[derive(Clone, Copy, Debug)]
pub struct ParseError {
    pub source_view:    SourceView,
    pub data:           ParseErrorData,
}

#[derive(Clone, Copy, Debug)]
pub enum ParseErrorData {
    UnexpectedEof,
    UnexpectedChar,
    Expected(TokenData<'static>),
    ExpectedExpression,
    TrailingInput,
    Unreachable(&'static str)
}

// The AST itself

struct Ast {
    
}