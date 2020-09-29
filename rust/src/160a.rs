mod utils;
use utils::{read_str, read_vec};

fn main() {
    read_str();

    let mut coins: Vec<u16> = read_vec(' ');
    let middle_sum: u16 = coins.iter().sum::<u16>() / 2;
    let mut minimum_sum: u16 = 0;
    let mut used_coins: u8 = 0;
    coins.sort_by(|a, b| b.cmp(a));

    for coin in coins {
        if minimum_sum <= middle_sum {
            minimum_sum += coin;
            used_coins += 1;
        }
        else { break; }
    }
    println!("{}", used_coins);
}