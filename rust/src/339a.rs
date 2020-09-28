mod utils;
use utils::read_vec;

fn main() {
    let mut numbers: Vec<u8> = read_vec('+');
    let mut expression: String = String::new();

    if numbers.len() > 1 {
        numbers.sort();
        expression.push_str(&numbers[0].to_string());
        for number in &numbers[1..] {
            expression.push('+');
            expression.push_str(&number.to_string());
        }
    }
    else {
        expression.push_str(&numbers[0].to_string());
    }
    println!("{}", expression);
}
