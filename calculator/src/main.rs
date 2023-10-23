use std::env::{Args, args};




fn main() {

    let mut arguments: Args =  args();

    let first = arguments.nth(1).unwrap();
    let operator: char = arguments.nth(0).unwrap().chars().next().unwrap();
    let second = arguments.nth(0).unwrap();


    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);
    println!("{}", output(first_number, operator, second_number, result));

    
}


fn operate(operation: char, frist: f32, second: f32) -> f32 {
    match operation {
        '+' => frist + second,
        '-' => frist - second,
        '/' => frist / second,
        'X' | '*' | 'x' => frist * second,
        _ => panic!("unsupported operation")
    }
}

fn output(frist: f32, operation: char, second: f32, result: f32) -> String {
    format!("{} {} {} = {}", frist, operation, second, result)
}