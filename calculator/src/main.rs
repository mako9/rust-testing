use std::io;

enum Operator {
    Plus,
    Minus,
    Multiplier,
    Divider
}

fn main() {
    println!("Enter first number:");
    let first = read_number();
    println!("Enter operator:");
    let operator = read_operator();
    println!("Enter second number:");
    let second = read_number();

    let result = calculate(first, second, operator);
    println!("Result: {}", result);
}

fn read_number() -> f32 {
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input.");
    
    text.trim().parse().expect("Failed to parse number.")
}

fn read_operator() -> Operator {
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input.");
    
    match text.trim() {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "*" => Operator::Multiplier,
        "/" => Operator::Divider,
        _ => panic!("No correct operator used: +, -, *, /")
    }
}

fn calculate(first: f32, second:  f32, operator: Operator) -> f32 {
    match operator {
        Operator::Plus => first + second,
        Operator::Minus => first - second,
        Operator::Multiplier => first * second,
        Operator::Divider => first / second
    }
}