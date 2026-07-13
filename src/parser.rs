//TODO: figure out a good way to flatten the AST instead of having a recursive Vec
#[derive(Debug, Clone)]
pub enum Token {
    Next,
    Prev,
    Incr,
    Decr,
    Out,
    In,
    Block(Vec<Token>),
}

//TODO: implement a parse from [std::io::Read]
#[inline(always)]
pub fn parse(input: &[u8]) -> Vec<Token> {
    parse_inner(input).0
}

fn parse_inner(input: &[u8]) -> (Vec<Token>, usize) {
    let mut ast = vec![];
    let len = input.len();
    let mut i = 0;
    while i < len {
        match input[i] {
            b'>' => ast.push(Token::Next),
            b'<' => ast.push(Token::Prev),
            b'+' => ast.push(Token::Incr),
            b'-' => ast.push(Token::Decr),
            b',' => ast.push(Token::In),
            b'.' => ast.push(Token::Out),
            b'[' => {
                let (children, offset) = parse_inner(&input[i + 1..len]);
                i += offset;
                ast.push(Token::Block(children));
            }
            b']' => {
                return (ast, i + 1);
            }
            _ => {}
        };
        i += 1;
    }

    (ast, 0)
}
