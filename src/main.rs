mod recursive_ascent;
mod recursive_descent;

fn main() {
    let test = "2 + 3 * 4
    100 + 100";

    let chars = test.chars().collect::<Vec<char>>();
    let mut lexer = recursive_descent::Lexer::new(&chars);
    lexer.lex();

    let mut parser = recursive_descent::Parser::new(&lexer.tokens);

    let ast = parser.parse().expect("Parse err");

    println!("{:?}", lexer.tokens);
    println!("{:?}", ast);

}