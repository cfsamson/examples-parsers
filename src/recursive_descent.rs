#[derive(Debug)]
pub enum Token {
    WhiteSpace,
    Number(char),
    Operator(char),
    NewLine,
}

// -> (--- number* space operator space number* ---)* newline | EOF

pub struct Lexer<'a> {
    text: &'a [char],
    pub(crate) tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {

    pub fn new(chars: &'a [char]) -> Self {
        Lexer {
            text: chars,
            tokens: vec![],
        }
    }
    pub fn lex(&mut self) {
        for c in self.text {
            self.parse_standard(*c);
        }
    }


    fn parse_standard(&mut self, c: char) {
        match c {
            '0'..='9' => {
                self.tokens.push(Token::Number(c));
            }

            '+' | '*' => {
                self.tokens.push(Token::Operator(c));
            }

            ' ' => self.tokens.push(Token::WhiteSpace),
            '\n' => self.tokens.push(Token::NewLine),

            _ => panic!("Invalid input: {}", c),
        }
    }
}


pub struct Parser<'a> {
    input: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token]) -> Self {
        Parser {
            input,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Ast, String> {
        let mut start = Ast::new(AstKind::Parent);
        while self.pos < self.input.len() {
            match self.parse_expr()? {
                Ast {kind: AstKind::Parent, ..} => {
                    start.add_child(self.parse()?);
                }

                ast @ _ => {
                    start.add_child(ast);
                }
            }
        
        }
        Ok(start)
    }

    fn parse_expr(&mut self) -> Result<Ast, String> {
        match self.input[self.pos] {
            Token::Number(c) => {
                let mut num: String = c.to_string();
                while self.advance() {
                    if let Token::Number(c) = self.input[self.pos] {
                        num.push(c);
                    } else {
                        break;
                    }
                }

                let num: f64 = num.parse().expect("Invalid number");

                return Ok(Ast::new(AstKind::Number(num)));
            }

            Token::Operator(c) => {
                let mut oper = c.to_string();

                while self.advance() {
                    if let Token::Operator(c) = self.input[self.pos] {
                        oper.push(c);
                    } else {
                        break;
                    }
                    
                }

                return Ok(Ast::new(AstKind::Operator(oper)));

            }

            Token::NewLine => {
                self.advance();
                return Ok(Ast::new(AstKind::Parent));
            }

            _ => {
                if self.advance() {
                    self.parse_expr()
                } else {
                    return Err(format!("END OF FILE"));
                }
            },
        }
    }

    // returns true if we're still within bounds
    fn advance(&mut self) -> bool {
        if self.pos + 1 < self.input.len() {
            self.pos += 1;
            true
        } else {
            self.pos += 1;
            false
        }
    }

}

#[derive(Debug)]
pub struct Ast {
    kind: AstKind,
    children: Vec<Ast>,
}

impl Ast {
    fn new(kind: AstKind) -> Self {
        Ast {
            kind,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Ast) {
        self.children.push(child);
    }
}

#[derive(Debug)]
enum AstKind {
    Parent,
    Number(f64),
    Operator(String),
}