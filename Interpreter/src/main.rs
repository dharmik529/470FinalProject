// Define Token and TokenType enums
#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    NumberType,
    DecimalType,
    ListType,
    WordsType,
    LetterType,
    LoopKeyword,
    FuncKeyword,
    FlagKeyword,
    IfKeyword,
    OtherwiseKeyword,
    StartKeyword,
    EndKeyword,
    CompleteKeyword,
    GiveBackKeyword,
    PrintKeyword,
    Plus,
    Minus,
    Mul,
    Div,
    Equal,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Assign,
    Comma,
    StringLiteral,
    Identifier,
    EOF,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}

// Define Lexer struct
pub struct Lexer {
    pub text: Vec<char>,
    pub pos: usize,
    pub current_char: Option<char>,
}

impl Lexer {
    fn new(text: &str) -> Lexer {
        let lexer = Lexer {
            text: text.chars().collect(),
            pos: 0,
            current_char: Some(text.chars().nth(0).unwrap_or('\0')),
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

    fn decimal(&mut self) -> String {
        let mut result = self.integer();
        if let Some(ch) = self.current_char {
            if ch == '.' {
                result.push(ch);
                self.advance();
                result.push_str(&self.integer());
            }
        }
        result
    }

    fn identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn string_literal(&mut self) -> String {
        let mut result = String::new();
        self.advance(); // Skip the opening single quote
        while let Some(ch) = self.current_char {
            if ch == '\'' {
                self.advance(); // Skip the closing single quote
                break;
            } else {
                result.push(ch);
                self.advance();
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
                let value = self.decimal();
                return Token {
                    token_type: TokenType::DecimalType,
                    value: Some(value),
                };
            }
            if ch.is_alphabetic() {
                let identifier = self.identifier();
                return match identifier.as_str() {
                    "number" => Token {
                        token_type: TokenType::NumberType,
                        value: None,
                    },
                    "decimal" => Token {
                        token_type: TokenType::DecimalType,
                        value: None,
                    },
                    "list" => Token {
                        token_type: TokenType::ListType,
                        value: None,
                    },
                    "words" => Token {
                        token_type: TokenType::WordsType,
                        value: None,
                    },
                    "letter" => Token {
                        token_type: TokenType::LetterType,
                        value: None,
                    },
                    "loop" => Token {
                        token_type: TokenType::LoopKeyword,
                        value: None,
                    },
                    "func" => Token {
                        token_type: TokenType::FuncKeyword,
                        value: None,
                    },
                    "flag" => Token {
                        token_type: TokenType::FlagKeyword,
                        value: None,
                    },
                    "if" => Token {
                        token_type: TokenType::IfKeyword,
                        value: None,
                    },
                    "otherwise" => Token {
                        token_type: TokenType::OtherwiseKeyword,
                        value: None,
                    },
                    "start" => Token {
                        token_type: TokenType::StartKeyword,
                        value: None,
                    },
                    "end" => Token {
                        token_type: TokenType::EndKeyword,
                        value: None,
                    },
                    "complete" => Token {
                        token_type: TokenType::CompleteKeyword,
                        value: None,
                    },
                    "give-back" => Token {
                        token_type: TokenType::GiveBackKeyword,
                        value: None,
                    },
                    "print" => Token {
                        token_type: TokenType::PrintKeyword,
                        value: None,
                    },
                    _ => Token {
                        token_type: TokenType::Identifier,
                        value: Some(identifier),
                    },
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
                '=' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Equal,
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
                '[' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::LBracket,
                        value: None,
                    };
                }
                ']' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::RBracket,
                        value: None,
                    };
                }
                '{' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::LBrace,
                        value: None,
                    };
                }
                '}' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::RBrace,
                        value: None,
                    };
                }
                '=' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Assign,
                        value: None,
                    };
                }
                ',' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Comma,
                        value: None,
                    };
                }
                '\'' => {
                    return Token {
                        token_type: TokenType::StringLiteral,
                        value: Some(self.string_literal()),
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

// Define AST nodes
#[derive(Debug)]
enum ASTNode {
    NumberLiteral(i32),
    DecimalLiteral(f64),
    StringLiteral(String),
    Identifier(String),
    BinaryExpr {
        left: Box<ASTNode>,
        operator: TokenType,
        right: Box<ASTNode>,
    },
    VariableDeclaration {
        var_type: TokenType,
        name: String,
        value: Box<ASTNode>,
    },
    PrintStatement(Box<ASTNode>),
    ListLiteral(Vec<ASTNode>),
    FunctionDeclaration {
        name: String,
        body: Box<ASTNode>,
    },
    BlockStatement(Vec<ASTNode>),
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn eat(&mut self, token_type: TokenType) {
        if self.current_token.token_type == token_type {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn factor(&mut self) -> ASTNode {
        let token = self.current_token.clone();
        match token.token_type {
            TokenType::NumberType => {
                self.eat(TokenType::NumberType);
                ASTNode::NumberLiteral(token.value.unwrap().parse().unwrap())
            }
            TokenType::DecimalType => {
                self.eat(TokenType::DecimalType);
                ASTNode::DecimalLiteral(token.value.unwrap().parse().unwrap())
            }
            TokenType::StringLiteral => {
                self.eat(TokenType::StringLiteral);
                ASTNode::StringLiteral(token.value.unwrap())
            }
            TokenType::Identifier => {
                self.eat(TokenType::Identifier);
                ASTNode::Identifier(token.value.unwrap())
            }
            TokenType::LParen => {
                self.eat(TokenType::LParen);
                let node = self.expr();
                self.eat(TokenType::RParen);
                node
            }
            TokenType::LBracket => {
                self.eat(TokenType::LBracket);
                let mut elements = Vec::new();
                while self.current_token.token_type != TokenType::RBracket {
                    elements.push(self.expr());
                    if self.current_token.token_type == TokenType::Comma {
                        self.eat(TokenType::Comma);
                    }
                }
                self.eat(TokenType::RBracket);
                ASTNode::ListLiteral(elements)
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    fn term(&mut self) -> ASTNode {
        let mut node = self.factor();

        while self.current_token.token_type == TokenType::Mul
            || self.current_token.token_type == TokenType::Div
        {
            let token = self.current_token.clone();
            if token.token_type == TokenType::Mul {
                self.eat(TokenType::Mul);
            } else if token.token_type == TokenType::Div {
                self.eat(TokenType::Div);
            }

            node = ASTNode::BinaryExpr {
                left: Box::new(node),
                operator: token.token_type,
                right: Box::new(self.factor()),
            };
        }

        node
    }

    fn expr(&mut self) -> ASTNode {
        let mut node = self.term();

        while self.current_token.token_type == TokenType::Plus
            || self.current_token.token_type == TokenType::Minus
        {
            let token = self.current_token.clone();
            if token.token_type == TokenType::Plus {
                self.eat(TokenType::Plus);
            } else if token.token_type == TokenType::Minus {
                self.eat(TokenType::Minus);
            }

            node = ASTNode::BinaryExpr {
                left: Box::new(node),
                operator: token.token_type,
                right: Box::new(self.term()),
            };
        }

        node
    }

    fn variable_declaration(&mut self) -> ASTNode {
        let var_type = self.current_token.token_type.clone();
        self.eat(var_type.clone());

        let name = self.current_token.value.clone().unwrap();
        self.eat(TokenType::Identifier);

        self.eat(TokenType::Equal);

        let value = self.expr();

        ASTNode::VariableDeclaration {
            var_type,
            name,
            value: Box::new(value),
        }
    }

    fn print_statement(&mut self) -> ASTNode {
        self.eat(TokenType::PrintKeyword);
        let value = self.expr();
        ASTNode::PrintStatement(Box::new(value))
    }

    fn function_declaration(&mut self) -> ASTNode {
        self.eat(TokenType::FuncKeyword);

        let name = self.current_token.value.clone().unwrap();
        self.eat(TokenType::Identifier);

        self.eat(TokenType::LBrace);
        let body = self.block_statement();
        self.eat(TokenType::RBrace);

        ASTNode::FunctionDeclaration {
            name,
            body: Box::new(body),
        }
    }

    fn block_statement(&mut self) -> ASTNode {
        let mut statements = Vec::new();

        while self.current_token.token_type != TokenType::RBrace {
            statements.push(self.statement());
        }

        ASTNode::BlockStatement(statements)
    }

    fn statement(&mut self) -> ASTNode {
        match self.current_token.token_type {
            TokenType::PrintKeyword => self.print_statement(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse(&mut self) -> ASTNode {
        match self.current_token.token_type {
            TokenType::NumberType
            | TokenType::DecimalType
            | TokenType::ListType
            | TokenType::WordsType
            | TokenType::LetterType => self.variable_declaration(),
            TokenType::PrintKeyword => self.print_statement(),
            TokenType::FuncKeyword => self.function_declaration(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }
}

fn main() {
    // Test case 1: Variable declaration with integer
    let lexer = Lexer::new("number myNum = 42");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 2: Variable declaration with decimal
    let lexer = Lexer::new("decimal myDecimal = 3.14");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 3: Variable declaration with string literal
    let lexer = Lexer::new("words myString = 'Hello, world!'");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 4: Variable declaration with list literal
    let lexer = Lexer::new("list myList = [1, 2, 3]");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 5: Print statement with string literal
    let lexer = Lexer::new("print 'Hello, world!'");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 6: Print statement with identifier
    let lexer = Lexer::new("print myVariable");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 7: Function declaration with print statement
    let lexer = Lexer::new("func myFunc { print 'Hello, world!' }");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 8: Binary expression with addition
    let lexer = Lexer::new("number result = 10 + 5");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 9: Binary expression with multiplication and division
    let lexer = Lexer::new("number result = 10 * 5 / 2");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);

    // Test case 10: Complex expression with parentheses
    let lexer = Lexer::new("number result = (10 + 5) * (3 - 1)");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);
}