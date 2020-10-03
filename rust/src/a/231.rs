use utils::{read_str, read_int};

fn main() {
    let mut total: u16 = 0;
    for _ in 0..read_int() {
        let sure_friends: u8 = read_str()
            .split_whitespace()
            .map(|friend| friend.parse::<u8>().expect("Error"))
            .sum();
        if sure_friends > 1 {
            total += 1;
        }
    }
    println!("{}", total);
}
