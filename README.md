## Description

A simple rust app calculator. User is prompted to write a basic expression to calculate. The result is outputed if expression is valid.

## Author
- Amir Nurmukhambetov (github.com/Hereugo)

## Features

- [x] Read user input
- [x] Checks for validity of a math expression using RPN
- [x] Process the expression, via:
    - [x] Convert to RPN expression
    - [x] Use stack to calculate it 
- [x] Output result
- [x] Handles basic math operations:
    - [x] addition
    - [x] subtraction
    - [x] multiplication
    - [x] division
    - [x] parantheses

## How to use:

First, you'll want to check out this repository

```shell
git clone https://github.com/hereugo/rusty-calculator-example
```

With cargo already installed, you can simply run:

```shell
cargo build --release
```

Then to run the program run:

```shell
./target/release/rusty_calculator # or .\target\release\rusty_calculator.exe on Windows
```

## Example:

```shell
Write any simple mathematic expression:
(1+2)*4-5
7
```
