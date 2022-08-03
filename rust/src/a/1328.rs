use utils::{read_int, read_tuple};

fn main() {
    let mut remainder: u32;
    for _ in 0..read_int() {
        let (a, b): (u32, u32) = read_tuple();
        remainder = a % b;
        if remainder != 0 { remainder = b - remainder; }
        print!("{}", remainder);
    }
}
