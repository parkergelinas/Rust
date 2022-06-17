use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse().unwrap();
    let result = operate(operator, first_number, second_number);


    println!("{:?}", result);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    if operator == '+' {
        return first_number + second_number;
    } else if operator == '-' {
        return first_number - second_number;
    } else if operator == '/' {
        return first_number / second_number;
    } else if operator == '*' {
        return first_number * second_number;
    } else {
        return 0.0;
    }
}