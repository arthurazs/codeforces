#[path = "../utils.rs"] mod utils;
use utils::read_vec;

fn main() {
    let (mut number, times): (u32, u8) = {
        let values: Vec<u32> = read_vec(' ');
        (values[0], values[1] as u8)
    };

    for _ in 0..times {
        if number % 10 == 0 { number /= 10; }
        else { number -= 1; }
    }

    println!("{}", number);
}