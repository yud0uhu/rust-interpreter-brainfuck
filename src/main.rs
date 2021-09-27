use std::error::Error;
mod token;
mod interpreter;
use token::lexer;
use interpreter::Interpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let my_str = include_str!("/home/denham/workspace/rust-interpreter-brainfuck/src/bf/hello.bf");
    let code = lexer(my_str);
    let interpreter = Interpreter::new(code);
    interpreter.run_with_stdio();
    Ok(())
}
