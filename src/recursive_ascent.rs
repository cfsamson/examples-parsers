fn test() {
    let test = "[1, [], 3]";
}


struct Lex {

}

impl Lex {
    fn token(&self) -> Token {
        unimplemented!();
    }
}

enum Token {
    Number(f64),
    LeftBracket,
    RightBracket,
    Comma,
    EndOfFile,
}

struct Value(JsonValue);
struct Array(Vec<JsonValue>);
struct Elements(Vec<JsonValue>);

enum JsonValue {
    Number(f64),
    Array(Vec<JsonValue>),
}

struct ParseError;