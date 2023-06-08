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

struct Ast {
    
}