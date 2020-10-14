use std::cmp::max;
use utils::{read_str, read_vec};

fn main() {
    let (mut previous, mut counter, mut maximum): (u64, u64, u64) = (0, 0, 0);
    read_str();
    for current in read_vec::<u64>(' ') {
        if current >= previous { counter += 1; }
        else { maximum = max(maximum, counter); counter = 1; }
        previous = current;
    }

    println!("{}", max(counter, maximum));
}