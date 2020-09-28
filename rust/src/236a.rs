mod utils;
use utils::read_str;

use std::collections::HashSet;

fn main() {
    let mut letters: HashSet<char> = HashSet::new();

    for letter in read_str().chars() {
        letters.insert(letter);
    }

    if letters.len() % 2 == 0 { println!("CHAT WITH HER!"); }
    else { println!("IGNORE HIM!"); }

}