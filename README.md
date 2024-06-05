# Tiny Basic Compiler

First attempt at implementing a simple compiler for a subset of the BASIC programming language, known as Tiny Basic.

## Features

- **Lexical Analysis**
- **Parsing**
- **Abstract Syntax Tree (AST) Generation**
- **Code Emission**

## Syntax Overview

### Notation

- `{}` - 0 or more
- `[]` - 0 or 1, i.e., optional
- `()` - grouping / or
- `+` - 1 or more

### Grammar

```
program ::= {statement}

statement ::= PRINT (expression | string) nl
              IF comparison "THEN" nl {statement} "ENDIF" nl
              WHILE comparison "REPEAT" nl {statement} "ENDWHILE" nl
              LABEL var nl
              GOTO var nl
              LET var "=" expression nl
              INPUT var nl

comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+

expression ::= term {( "-" | "+" ) term}

term ::= unary {( "/" | "*" ) unary}

unary ::= ["+" | "-"] primary

primary ::= number | var
```


## Sample Program

Here's a sample Tiny Basic program:

```basic
PRINT "How many fibonacci numbers do you want?"
INPUT nums
PRINT ""

LET a = 0
LET b = 1
WHILE nums > 0 REPEAT
    PRINT a
    LET c = a + b
    LET a = b
    LET b = c
    LET nums = nums - 1
ENDWHILE

```

## Getting Started

1. **Clone the repository**
    ```sh
    git clone https://github.com/your-repo/tiny-basic-compiler.git
    cd tiny-basic-compiler
    ```

2. **Run the compiler**
    ```sh
    just compile sample_input.tb
    ```

3. **To see generated Abstract syntax tree**
   ```sh
   just debug sample_input.tb
   ```

## File Structure

```
📂 src
 ┣ 📜 main.rs       // Entry point
 ┣ 📜 lexer.rs      // Tokenization logic
 ┣ 📜 lib.rs        // Shared utilities
 ┣ 📜 emitter.rs    // Code emission logic
 ┣ 📜 ast.rs        // Abstract Syntax Tree structures
 ┗ 📜 parser.rs     // Parsing logic
```
