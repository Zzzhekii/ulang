use ulang;

fn main() {
//     let source = "-- An example '99 bottles of bear' program.

// const AMOUNT_OF_BOTTLES = 99

// static main = fn
//     let current_amount = AMOUNT_OF_BOTTLES
//     while current_amount > 0 do
//         println '{} bottles of beer on the wall, {} bottles of beer.', current_amount, current_amount
//         println 'Take one down and pass it around, {}', do
//             current_amount -= 1
//             if current_amount == 0
//                 'no more bottles of beer on the wall.'
//             else
//                 format '{} bottles of beer on the ball.', current_amount
//             end
//         end
//     end

//     println 'No more bottles of beer on the wall, no more bottles of beer.'
//     println 'Go to the store and buy some more, {} bottles of beer on the wall.', AMOUNT_OF_BOTTLES
// end";

//     let tokens = ulang::compiler::parser::Tokenizer::tokenize(source.as_bytes());
//     for t in tokens.unwrap() {
//         println!("{:?}", t);
//     }
    
    //println!("{:?}", ulang::bytecode::new_header())

    let vm = ulang::vm::vm::VM::new();
    vm.run();
}