use std::io;

fn get_priority(chr: &char) -> i32 {
    match chr {
        '(' => 0,
        '+' | '-' => 1,
        _ => 2,
    }
}

fn convert_to_rpn(expr: &String) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut result: String = String::new();

    for token in expr.chars() {
        match token {
            '+' | '-' | '*' | '/' => {
                while !stack.is_empty()
                    && get_priority(stack.last().unwrap()) >= get_priority(&token)
                {
                    result.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
            '(' => stack.push(token),
            ')' => {
                while *stack.last().unwrap() != '(' {
                    result.push(stack.pop().unwrap());
                }
                stack.pop();
            }
            '0'..='9' => result.push(token),
            _ => panic!("Couldnt process symbol {token}"),
        }
    }

    while !stack.is_empty() {
        result.push(stack.pop().unwrap());
    }

    result
}

fn main() {
    let mut input = String::new();

    println!("Write any simple mathematic expression: ");

    io::stdin().read_line(&mut input).expect("Some error");

    let expr: String = input.trim().parse().expect("Should be a string");

    let expr = convert_to_rpn(&expr);

    // println!("RPN expression: {}", expr);

    let mut stack: Vec<f32> = Vec::new();
    for token in expr.chars() {
        if '0' <= token && token <= '9' {
            stack.push(token.to_digit(10).unwrap() as f32);
            continue;
        }

        let res;
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();

        match token {
            '+' => res = a + b,
            '-' => res = b - a,
            '*' => res = a * b,
            '/' => res = b / a,
            _ => panic!("Couldnt process symbol {token}"),
        }

        stack.push(res);
    }

    if stack.len() != 1 {
        panic!("Math expression evaluation resulted in a wrong output.");
    }

    let result = stack.last().unwrap();

    println!("{}", result);
}
