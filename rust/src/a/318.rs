use utils::read_tuple;

fn main() {
    let (k, n): (u64, u64) = read_tuple();
    let mut mid: u64 = k / 2;

    if k % 2 == 1 { mid += 1; }

    if n > mid { println!("{}", 2 * (n - mid)); }
    else { println!("{}", (2 * n) - 1); }
}
