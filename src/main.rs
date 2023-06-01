mod token;
mod lexer;
mod parser;
mod ast;

fn main() {
    let mut l = lexer::Lexer::from_str(
"fn main do
    print \'Hello, World\'
    print (12+4)*3 + 0b123
end"
    );

    let ast = parser::parse(&mut l); 

    while let Some(t) = l.next() {
        println!("Position: {}, {}; Token: {:?}", t.row, t.col, t.t_type);
    }
}