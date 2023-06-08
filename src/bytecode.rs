const BYTECODE_CONVENTION_VERSION: u32 = 0;                                     // Version of the bytecode convention
const BYTECODE_CONVENTION_COMPILER_INTRODUCTION_VERSION: &str = "0.0.1";        // Version of the compiler, in which the convention was introduced

/*
Header structure:

// NOTE:    String are null-terminated!!!
// NOTE2:   All integers are stored as BIG ENDIAN!!!

/====HEADER SECTION====/

(u32):      [BYTECODE_CONVENTION_VERSION]                       // This part of the header must come first in 
                                                                // every convention verison (for backwards compatibility)
(string):   [BYTECODE_CONVENTION_COMPILER_INTRODUCTION_VERSION] //

/====CODE SECTION=====/
  ...
*/

use std::mem::transmute;

pub fn new_header<'h>() -> &'h [u8] {
    let mut header: Vec<u8> = vec![];

    // Convention version
    unsafe { transmute::<u32, [u8; 4]>(BYTECODE_CONVENTION_VERSION.to_be()) }
        .iter()
        .for_each(|e| header.push(*e));

    // Convention introduction version
    BYTECODE_CONVENTION_COMPILER_INTRODUCTION_VERSION
        .as_bytes()
        .iter()
        .for_each(|e| header.push(*e));

    // Terminate the string
    header.push('\0' as u8);



    header.leak()
}

/*
General insturction layout:
- Like MOVE, etc...
    [OPCODE] [DEST] [ORIGIN]

- Like ADD, SUB, etc...
    [OPCODE] [DEST] [ARGS ORIGINS]

- Like GOTO, CALL, etc...
    [OPCODE] [ARGS]
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Opcode {
    Nop     = 0b00001_000,
    Copy    = 0b00010_000,
    Move    = 0b00011_000,
    Add     = 0b00100_000,
    Sub     = 0b00101_000,    // Substruct
    Goto    = 0b00110_000,
    Call    = 0b00111_000,
}

impl Opcode {
    fn to_u8(&self) -> u8 {
        *self as u8
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::Nop   => "NOP",
            Self::Copy  => "COPY",
            Self::Move  => "MOVE",
            Self::Add   => "ADD",
            Self::Sub   => "SUB",
            Self::Goto  => "GOTO",
            Self::Call  => "CALL",
        }
    }
}