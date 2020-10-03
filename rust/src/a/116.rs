use utils::{read_int, read_tuple};

fn main() {
    let mut minimum_capacity: u32 = 0;
    let mut current_capacity: u32 = 0;
    for _ in 0..read_int::<u32>() {
        let (a, b): (u32, u32) = read_tuple();
        current_capacity = current_capacity - a + b;
        minimum_capacity = std::cmp::max(minimum_capacity, current_capacity);
    }
    println!("{}", minimum_capacity);
}