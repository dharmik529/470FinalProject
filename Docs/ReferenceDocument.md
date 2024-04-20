# Symplnym Language Reference Manual

This document serves as a quick reference for the Symplnym programming language, describing the available statements and syntax variations for use.

## Class Constructors and Methods

### Variable Class
| Syntax                  | Description                                                                           |
|-------------------------|---------------------------------------------------------------------------------------|
| `number(variable_name)` | Constructs a variable that represents an integer and assigns it to `variable_name`.   |
| `decimal(variable_name)`| Constructs a variable that represents a decimal and assigns it to `variable_name`.    |
| `words(variable_name)`  | Constructs a variable that represents a string and assigns it to `variable_name`.     |
| `list(variable_name)`   | Constructs a variable that represents a list and assigns it to `variable_name`.       |

### Operations
| Syntax                | Description                                                                            |
|-----------------------|----------------------------------------------------------------------------------------|
| `+`                   | Adds two numbers.                                                                      |
| `-`                   | Subtracts the second number from the first.                                            |
| `*`                   | Multiplies two numbers.                                                                |
| `/`                   | Divides the first number by the second.                                                |

### Control Structures
| Syntax                 | Description                                                                            |
|------------------------|----------------------------------------------------------------------------------------|
| `if (condition)`       | Evaluates a condition; if true, the following block is executed.                       |
| `otherwise`            | If the preceding `if` condition is false, the following block is executed.             |

### Functions
| Syntax                 | Description                                                                            |
|------------------------|----------------------------------------------------------------------------------------|
| `func function_name()` | Declares a new function with the name `function_name`.                                 |

### Printing
| Syntax                 | Description                                                                            |
|------------------------|----------------------------------------------------------------------------------------|
| `print(value)`         | Outputs `value` to the console.                                                        |

### List Operations
| Syntax                 | Description                                                                            |
|------------------------|----------------------------------------------------------------------------------------|
| `list.add(value)`      | Appends `value` to the end of the list; returns true.                                  |
| `list.get(index)`      | Retrieves the element at `index` in the list.                                          |

### Comparison and Logic
| Syntax                 | Description                                                                            |
|------------------------|----------------------------------------------------------------------------------------|
| `==`                   | Compares two values for equality.                                                      |
| `!=`                   | Compares two values for inequality.                                                    |
| `<`                    | Checks if the first value is less than the second.                                     |
| `>`                    | Checks if the first value is greater than the second.                                  |
| `<=`                   | Checks if the first value is less than or equal to the second.                         |
| `>=`                   | Checks if the first value is greater than or equal to the second.                      |

