use utils::{read_tuple, Utils};

fn main() {
    let (k, n): (u64, u64) = read_tuple();
    let mid: u64 = k.div_ceil(2);

    if n > mid { println!("{}", 2 * (n - mid)); }
    else { println!("{}", (2 * n) - 1); }
}
