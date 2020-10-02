#[path = "../utils.rs"] mod utils;
use utils::{read_int, div_ceil};

fn main() {
    println!("{}", div_ceil(read_int(), 5));
}