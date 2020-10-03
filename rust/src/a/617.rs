use utils::{read_int, Utils};

fn main() {
    println!("{}", read_int::<u64>().div_ceil(5u64));
}