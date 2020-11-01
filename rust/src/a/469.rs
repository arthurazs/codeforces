use utils::{read_int, read_vec};
use std::collections::HashSet;

fn main() {
    let n: usize = read_int();
    let mut total: HashSet<u8> = HashSet::new();

    for _ in 0..2 {
        for &value in &read_vec(' ')[1..] {
            total.insert(value);
        }
    }

    if n == total.len() { print!("I become the guy."); }
    else { print!("Oh, my keyboard!"); }
}