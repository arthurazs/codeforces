use utils::{read_int};

fn main() {
    let n: u64 = read_int();

    if n % 2 == 0 { println!("{}", n / 2); }
    else { println!("-{}", (n + 1) / 2); }
}
