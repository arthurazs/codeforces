#[path = "../utils.rs"] mod utils;
use utils::{read_vec, div_ceil};

fn main() {
    let (n, m, a): (u64, u64, u64) = {
        let values: Vec<u64> = read_vec(' ');
        (values[0], values[1], values[2])
    };

    println!("{}", div_ceil(n, a) * div_ceil(m, a));
}
