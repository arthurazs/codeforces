#[path = "../utils.rs"] mod utils;
use utils::read_str;

fn main() {
    read_str();
    let mut remove: u8 = 0;
    let stones: String = read_str();
    let mut current: char = stones.chars().nth(0).expect("Error");

    for stone in stones[1..].chars() {
        if current == stone { remove += 1; }
        current = stone;
    }
    println!("{}", remove)
}