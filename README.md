# 470FinalProject

# Final Project Submission

## 1. List of the Names and Sections Numbers of Group Members
- Cameron Bussom (Section 002)
- Dharmik Patel (Section 001)

## 2. Final Version of Language Whitepaper and Language Specification Documentation

### Language Whitepaper
The language, named "Symplnym" is a simple programming language designed for educational purposes. It features a minimal set of keywords and constructs to facilitate learning and understanding of basic programming concepts.

### Language Specification
- Symplnym is a statically-typed language with a simple syntax.
- It supports variable declarations, assignments, arithmetic operations, and basic control flow statements.
- The language uses a lexer and parser to analyze the source code and generate an Abstract Syntax Tree (AST).
- The AST is then traversed to execute the program.

## 3. Language Tutorial with at Least One Example Exercise

### Tutorial
Symplnym is a beginner-friendly programming language. Here's a simple tutorial to get started:

1. Variable Declaration:
   - Use the keywords `number`, `decimal`, `words`, `list`, or `letter` to declare variables of different types.
   - Example: `number myNum = 42`

2. Arithmetic Operations:
   - MyLang supports basic arithmetic operations like addition, subtraction, multiplication, and division.
   - Example: `number result = 10 + 5`

3. Control Flow:
   - MyLang provides an `if` statement for conditional execution.
   - Example: `if (myNum > 0) { print 'Positive' }`

### Example Exercise
Write a Symplnym program that calculates the sum of two numbers and prints the result.
- number num1 = 10
- number num2 = 20
- number sum = num1 + num2
- print 'The sum is: '
- print sum

## 4. Language Reference Manual - Language Key Words and How to Use Them Syntactically

### Keywords
- `number`: Declares an integer variable.
- `decimal`: Declares a floating-point variable.
- `words`: Declares a string variable.
- `list`: Declares a list variable.
- `letter`: Declares a character variable.
- `loop`: Defines a loop construct.
- `func`: Defines a function.
- `flag`: Declares a boolean variable.
- `if`: Defines a conditional statement.
- `otherwise`: Defines an else branch of a conditional statement.
- `start`: Marks the beginning of a block.
- `end`: Marks the end of a block.
- `complete`: Indicates the completion of a statement or block.
- `give-back`: Returns a value from a function.
- `print`: Prints a value to the console.

### Syntax
- Variable Declaration: `<type> <variable_name> = <value>`
- Arithmetic Operations: `<variable> = <expression>`
- Function Declaration: `func <function_name> { <statements> }`
- Conditional Statement: `if (<condition>) { <statements> }`
- Print Statement: `print <value>`

## 5. Project Development Methodology, Plan and Estimation Schedule
- Development Methodology: Agile
- Project Plan:
  - Week 1: Requirements gathering and language design
  - Week 2: Lexer and parser implementation
  - Week 3: AST and interpreter development
  - Week 4: Testing and documentation
- Estimated Timeline:
  - Language Design: 5 days
  - Lexer and Parser: 7 days
  - AST and Interpreter: 10 days
  - Testing and Documentation: 5 days

## 6. Instructions to Download and Install the Final Version of the Final Project
1. Clone the project repository: `git clone https://github.com/username/project.git`
2. Install Rust programming language: [Rust Installation Guide](https://www.rust-lang.org/tools/install)
3. Navigate to the project directory: `cd 470/finalproject/interpreter`
4. Build the project: `cargo build`
5. Run the project: `cargo run`

## 7. Link to Access the Final Version of the Implemented Software
- GitHub Repository: [https://github.com/dharmik529/470FinalProject](https://github.com/dharmik529/470FinalProject)

## 8. Conclusions - Post Mortem Documentation (By Team) and (Individual)

### Team Conclusions
- The project was successfully completed within the estimated timeline.
- The language design phase took longer than expected due to discussions and iterations.
- The implementation phase went smoothly thanks to the well-defined language specifications.
- Testing and documentation required more effort than initially anticipated.

### Individual Conclusions
- Cameron Bussom: I learned a lot about language design and parsing techniques during this project.
- Dharmik Patel: Implementing the interpreter was a challenging but rewarding experience.

