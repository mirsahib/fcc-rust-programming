use std::env;

fn main() {
    let args: Vec<String>= env::args().collect();
    let first = &args[1];
    let operator = &args[2].chars().next().unwrap();
    let second = &args[3];

    let first_num: f32 = first.parse().unwrap();
    let second_num: f32 = second.parse().unwrap();
    let result = operate(*operator,first_num,second_num);
    output(first_num, *operator, second_num, result);
}
fn operate(operator: char, first: f32, second: f32)->f32{
    match operator {
        '+' => first + second,
        '-' => first - second,
        '*'| 'x' | 'X' => first * second,
        '/' => first / second,
        _ => panic!("Invalid operator"),
    }
}

fn output(first_num:f32,operator: char,second_num:f32,result:f32){
    println!("{} {} {} = {}", first_num,operator,second_num,result);
}