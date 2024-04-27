# Test Plan

## Objective
The objective of this test plan is to ensure the correctness and functionality of the lexer and parser implementation for the Symplnym programming language.

## Scope
The test plan covers the following components:
- Lexer: Tokenization of input strings
- Parser: Parsing of tokens into an Abstract Syntax Tree (AST)

## Test Cases
1. Variable Declaration:
   - Test Case 1.1: Integer variable declaration
   - Test Case 1.2: Decimal variable declaration
   - Test Case 1.3: String variable declaration
   - Test Case 1.4: List variable declaration

2. Print Statement:
   - Test Case 2.1: Print statement with string literal
   - Test Case 2.2: Print statement with identifier

3. Function Declaration:
   - Test Case 3.1: Function declaration with print statement

4. Binary Expression:
   - Test Case 4.1: Binary expression with addition
   - Test Case 4.2: Binary expression with multiplication and division
   - Test Case 4.3: Complex expression with parentheses

5. Comparison Expression:
   - Test Case 5.1: Less than comparison

## Expected Results
1. Variable Declaration:
   - Test Case 1.1: AST node representing an integer variable declaration
   - Test Case 1.2: AST node representing a decimal variable declaration
   - Test Case 1.3: AST node representing a string variable declaration
   - Test Case 1.4: AST node representing a list variable declaration

2. Print Statement:
   - Test Case 2.1: AST node representing a print statement with a string literal
   - Test Case 2.2: AST node representing a print statement with an identifier

3. Function Declaration:
   - Test Case 3.1: AST node representing a function declaration with a print statement

4. Binary Expression:
   - Test Case 4.1: AST node representing a binary expression with addition
   - Test Case 4.2: AST node representing a binary expression with multiplication and division
   - Test Case 4.3: AST node representing a complex expression with parentheses

5. Comparison Expression:
   - Test Case 5.1: AST node representing a less than comparison

# Test Scripts

## Test Script 1: Variable Declaration
​```rust
// Test case 1.1: Variable declaration with integer
let lexer = Lexer::new("number myNum = 42");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);

// Test case 1.2: Variable declaration with decimal
let lexer = Lexer::new("decimal myDecimal = 3.14");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);

// Test case 1.3: Variable declaration with string literal
let lexer = Lexer::new("words myString = 'Hello, world!'");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);

// Test case 1.4: Variable declaration with list literal
let lexer = Lexer::new("list myList = [1, 2, 3]");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);
​```

## Test Script 2: Print Statement
​```rust
// Test case 2.1: Print statement with string literal
let lexer = Lexer::new("print 'Hello, world!'");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);

// Test case 2.2: Print statement with identifier
let lexer = Lexer::new("print myVariable");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);
​```

## Test Script 3: Function Declaration
​```rust
// Test case 3.1: Function declaration with print statement
let lexer = Lexer::new("func myFunc { print 'Hello, world!' }");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);
​```

## Test Script 4: Binary Expression
​```rust
// Test case 4.1: Binary expression with addition
let lexer = Lexer::new("number result = 10 + 5");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);

// Test case 4.2: Binary expression with multiplication and division
let lexer = Lexer::new("number result = 10 * 5 / 2");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);

// Test case 4.3: Complex expression with parentheses
let lexer = Lexer::new("number result = (10 + 5) * (3 - 1)");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);
​```

## Test Script 5: Comparison Expression
​```rust
// Test case 5.1: Less than comparison
let lexer = Lexer::new("number result = 10 < 20");
let mut parser = Parser::new(lexer);
let ast = parser.parse();
println!("{:?}", ast);
​```
