use utils::read_int;

fn main() {
    let (k, l, m, n, d): (u32, u32, u32, u32, u32) = (
        read_int(), read_int(), read_int(), read_int(), read_int());

    let mut total: u32 = 0;
    for index in 1..d + 1 {
        if index % k == 0 { total += 1; }
        else if index % l == 0 { total += 1; }
        else if index % m == 0 { total += 1; }
        else if index % n == 0 { total += 1; }
    }
    print!("{}", total);
}