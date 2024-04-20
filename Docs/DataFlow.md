# Define Token and TokenType
Define enum TokenType with values for various tokens like NumberType, DecimalType, etc.
Define struct Token with fields for token_type (TokenType) and value (optional string).

# Define Lexer
Define struct Lexer with fields for text (array of characters), position, and current character.
Define methods for Lexer:
  - new(text): Initialize lexer with text.
  - advance(): Move to the next character.
  - skip_whitespace(): Skip whitespace characters.
  - integer(): Return a string representing a whole number.
  - decimal(): Return a string representing a decimal number.
  - identifier(): Return a string representing an identifier.
  - string_literal(): Return a string enclosed in quotes.
  - get_next_token(): Determine the next token based on the current character.

# Define AST Nodes
Define enum ASTNode for different types of abstract syntax tree nodes like NumberLiteral, DecimalLiteral, etc.
Define struct Parser with fields for lexer and current token.
Define methods for Parser:
  - new(lexer): Initialize parser with a lexer.
  - eat(expected_token_type): Consume a token of the expected type or panic.
  - factor(), term(), expr(): Methods to parse expressions based on precedence.
  - variable_declaration(), print_statement(), function_declaration(), block_statement(), statement(): Methods to parse different kinds of statements.
  - parse(): Parse the input from the lexer into an AST.

# Main Function
In the main function:
  - Initialize a lexer with some test input.
  - Initialize a parser with this lexer.
  - Parse the input and print the AST.
