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

fn opcode_list() -> Vec<(Opcode, u8, &'static str)> {
    vec![
        (Opcode::Nop,   0b00001_000, "NOP"),
        (Opcode::Copy,  0b00010_000, "COPY"),
        (Opcode::Move,  0b00011_000, "MOVE"),
        (Opcode::Add,   0b00100_000, "ADD"),
        (Opcode::Sub,   0b00101_000, "SUB"),
        (Opcode::Goto,  0b00110_000, "GOTO"),
        (Opcode::Call,  0b00111_000, "CALL"),
    ]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Opcode {
    Nop,
    Copy,
    Move,
    Add,
    Sub,    // Substruct
    Goto,
    Call,
}

impl Opcode {
    pub fn from_u8(byte: u8) -> Self {
        for (op, b, _) in opcode_list() {
            if byte == b {
                return op
            }
        }

        panic!("[BUG] Could not convert byte to an opcode. Got: {}.", byte)
    }

    pub fn to_u8(&self) -> u8 {
        for (op, b, _) in opcode_list() {
            if *self == op {
                return b;
            }
        }

        panic!("[BUG] Could not convert opcode to a byte. Got: {:?}.", *self)
    }

    pub fn to_str(&self) -> &str {
        for (op, _, string) in opcode_list() {
            if *self == op { 
                return string;
            }
        }

        panic!("[BUG] Could not convert opcode to a string. Got: {:?}.", *self)
    }
}