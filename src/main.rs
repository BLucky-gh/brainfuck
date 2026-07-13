mod interpreter;
mod parser;

fn main() {
    let program = include_str!("./program.bf");
    //
    // let program = "-a-a-a[a+a+dad+a+a]a+f--f+";
    let ast = parser::parse(program.as_bytes());
    // println!("{ast:#?}");

    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(&ast);
}
