mod utils;
use utils::{read_int, read_vec};

fn main() {
    let mut minimum_capacity: u32 = 0;
    let mut current_capacity: u32 = 0;
    for stop in 0..read_int::<u32>() {
        let (a, b): (u32, u32) = {
            let values: Vec<u32> = read_vec(' ');
            (values[0], values[1])
        };
        current_capacity = current_capacity - a + b;
        minimum_capacity = std::cmp::max(minimum_capacity, current_capacity);
    }
    println!("{}", minimum_capacity);
}