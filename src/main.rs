use std::str::FromStr;

use color_eyre::eyre::bail;

use self::{interpreter::Interpreter, parser::Ast};

mod interpreter;
mod parser;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    // let mut buf = Vec::with_capacity(100);
    // let mut stdin = io::stdin().lock();

    // stdin.read_to_end(&mut buf)?;

    // let buf = String::from_utf8(buf)?;

    let ast = match Ast::from_str("+++++++[>+++++++++>+++++++<<-]>---.>++.") {
        Ok(v) => v,
        Err(m) => bail!(m),
    };

    let input: Vec<u8> = vec![1, 2, 3];
    let mut output: Vec<u8> = Vec::new();

    let mut interpreter = Interpreter::new(&ast, input.as_slice(), &mut output);
    interpreter.interpret_all();

    println!("Output:\n{}", String::from_utf8(output).unwrap());

    Ok(())
}
