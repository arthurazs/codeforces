use utils::{read_tuple};

fn main() {
    let (m, n): (u16, u16) = read_tuple();
    println!("{}", (m * n) / 2);
}
