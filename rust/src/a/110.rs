#[path = "../utils.rs"] mod utils;
use utils::read_str;

fn main() {
    let mut lucky_digits: u8 = 0;

    for digit in read_str().chars() {
        if digit == '4' || digit == '7' { lucky_digits += 1; }
    }

    if lucky_digits == 4 || lucky_digits == 7 { println!("YES"); }
    else { println!("NO"); }
}