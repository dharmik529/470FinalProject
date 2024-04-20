// Define Token and TokenType enums
#[derive(Debug, PartialEq)]
enum TokenType {
    Integer,
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
    EOF,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}

// Define Lexer struct
struct Lexer {
    text: Vec<char>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    fn new(text: &str) -> Lexer {
        let mut lexer = Lexer {
            text: text.chars().collect(),
            pos: 0,
            current_char: Some(text.chars().nth(0).unwrap()),
        };
        lexer
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.text.len() {
            self.current_char = Some(self.text[self.pos]);
        } else {
            self.current_char = None;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn integer(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_digit(10) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn get_next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            if ch.is_digit(10) {
                return Token {
                    token_type: TokenType::Integer,
                    value: Some(self.integer()),
                };
            }
            match ch {
                '+' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Plus,
                        value: None,
                    };
                }
                '-' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Minus,
                        value: None,
                    };
                }
                '*' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Mul,
                        value: None,
                    };
                }
                '/' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Div,
                        value: None,
                    };
                }
                '(' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::LParen,
                        value: None,
                    };
                }
                ')' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::RParen,
                        value: None,
                    };
                }
                _ => panic!("Invalid character"),
            }
        }
        Token {
            token_type: TokenType::EOF,
            value: None,
        }
    }
}

fn main() {
    let mut lexer = Lexer::new("3 + 5 * (10 - 6)");
    let mut token = lexer.get_next_token();

    loop {
        println!("{:?}", token);
        if token.token_type == TokenType::EOF {
            break;
        }
        token = lexer.get_next_token();
    }
}

