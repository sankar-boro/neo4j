use std::iter::Peekable;
use std::str::Chars;

// Define tokens for the Cypher language
#[derive(Debug, PartialEq)]
enum Token {
    Match,
    LeftParenthesis,
    RightParenthesis,
    Dash,
    LeftBracket,
    RightBracket,
    Arrow,
    Variable(String),
    Comma,
    Semicolon,
    Whitespace,
}

// Define a simple lexer to tokenize the input Cypher query
struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.input.peek() {
            Some(&'M') => {
                self.input.next(); // Consume 'M'
                Some(Token::Match)
            }
            Some(&'(') => {
                self.input.next(); // Consume '('
                Some(Token::LeftParenthesis)
            }
            Some(&')') => {
                self.input.next(); // Consume ')'
                Some(Token::RightParenthesis)
            }
            Some(&'-') => {
                self.input.next(); // Consume '-'
                Some(Token::Dash)
            }
            Some(&'[') => {
                self.input.next(); // Consume '['
                Some(Token::LeftBracket)
            }
            Some(&']') => {
                self.input.next(); // Consume ']'
                Some(Token::RightBracket)
            }
            Some(&'-') => {
                self.input.next(); // Consume '->'
                self.input.next(); // Consume '>'
                Some(Token::Arrow)
            }
            Some(&',') => {
                self.input.next(); // Consume ','
                Some(Token::Comma)
            }
            Some(&';') => {
                self.input.next(); // Consume ';'
                Some(Token::Semicolon)
            }
            Some(&c) if c.is_whitespace() => {
                while let Some(&c) = self.input.peek() {
                    if c.is_whitespace() {
                        self.input.next();
                    } else {
                        break;
                    }
                }
                Some(Token::Whitespace)
            }
            Some(_) => {
                let mut variable = String::new();
                while let Some(&c) = self.input.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        variable.push(c);
                        self.input.next();
                    } else {
                        break;
                    }
                }
                if !variable.is_empty() {
                    Some(Token::Variable(variable))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

// Define a basic Cypher parser
struct CypherParser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> CypherParser<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        println!("current_token: {:?}", current_token);
        CypherParser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn eat(&mut self, expected: Token) {
        if self.current_token == Some(expected) {
            self.advance();
        } else {
            // panic!("Expected {:?}, found {:?}", expected, self.current_token);
        }
    }

    fn match_clause(&mut self) {
        self.eat(Token::Match);
        self.eat(Token::LeftParenthesis);
        self.node_pattern();
        self.eat(Token::Dash);
        self.eat(Token::LeftBracket);
        self.eat(Token::Variable("r".to_owned())); // Relationship variable
        self.eat(Token::RightBracket);
        self.eat(Token::Arrow);
        self.node_pattern();
        self.eat(Token::RightParenthesis);
    }

    fn node_pattern(&mut self) {
        self.eat(Token::LeftParenthesis);
        self.eat(Token::Variable("n".to_owned())); // Node variable
        self.eat(Token::RightParenthesis);
    }

    // Implement more parsing functions for additional Cypher grammar rules as needed

    fn parse(&mut self) {
        while self.current_token.is_some() {
            match self.current_token {
                Some(Token::Match) => self.match_clause(),
                _ => panic!("Unexpected token: {:?}", self.current_token),
            }
        }
    }
}

fn main() {
    let cypher_query = "MATCH (n)-[r]->(m) RETURN n, r, m;";
    let mut parser = CypherParser::new(cypher_query);
    parser.parse();
}