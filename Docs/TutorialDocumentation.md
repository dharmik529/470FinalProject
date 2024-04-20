# Symplnym Language Tutorial Documentation

Welcome to the official tutorial for Symplnym, a flexible and powerful programming language designed to provide clear and efficient coding capabilities. This tutorial will guide you through the basics of using Symplnym, with an emphasis on its various statements and the syntax you'll use to write effective code.

## Language Overview

Symplnym offers a straightforward syntax that closely resembles natural English, which helps in making the code easy to read and understand. It supports various data types like numbers, decimals, lists, words, and more. It also includes a variety of operators for arithmetic and logic operations, alongside control structures to manage the flow of execution.

## Basic Syntax and Statements

### Variable Declaration
Variables in Symplnym can be declared using specific keywords depending on the data type:

- `number`: for integers
- `decimal`: for floating-point numbers
- `words`: for strings
- `list`: for arrays

**Example:**

```
number myNum = 42
decimal myDecimal = 3.14
words myString = 'Hello, world!'
list myList = [1, 2, 3]
```
### Print Statements
To output data to the console, use the `print` keyword.

**Example:**
```
print 'Hello, world!'
```
### Arithmetic Operations
Symplnym supports basic arithmetic operations such as addition, subtraction, multiplication, and division.

**Example:**
```
number result = 10 + 5
print result // Outputs 15
```
### Control Structures
Symplnym provides conditional statements such as `if` and `otherwise`.

**Example:**
```
number age = 18
if (age >= 18) {
print 'You are an adult.'
} otherwise {
print 'You are a minor.'
}
```
## Example Exercise: Simple Calculator

In this exercise, you will create a simple calculator that can perform addition, subtraction, multiplication, and division based on user input.

### Step 1: Variable Declarations
Declare the variables for storing numbers and the operation.
```
number num1 = 10
number num2 = 5
words operation = 'add'
```
### Step 2: Implementing Operations
Use conditional statements to perform operations based on the operation variable.
```
if (operation == 'add') {
number result = num1 + num2
print 'Addition Result: '
print result
} otherwise if (operation == 'subtract') {
number result = num1 - num2
print 'Subtraction Result: '
print result
} otherwise if (operation == 'multiply') {
number result = num1 * num2
print 'Multiplication Result: '
print result
} otherwise if (operation == 'divide') {
decimal result = num1 / num2
print 'Division Result: '
print result
} otherwise {
print 'Invalid operation'
}
```

### Step 3: Running Your Code
After setting up your code, execute it and you should see the output based on the operation specified.
## Conclusion

This tutorial provided a basic introduction to the Symplnym programming language, covering variable declarations, arithmetic operations, print statements, and control structures. By completing the simple calculator exercise, you've taken your first steps into programming with Symplnym. Keep experimenting with different operations and control structures to deepen your understanding of the language.
