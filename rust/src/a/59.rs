#[path = "../utils.rs"] mod utils;
use utils::read_str;

fn main() {
    let word: String = read_str();

    let mut lower: i8 = 0;

    for letter in word.chars() {
        if letter.is_lowercase() { lower += 1; }
        else { lower -= 1; }
    }

    if lower >= 0 { println!("{}", word.to_lowercase()); }
    else { println!("{}", word.to_uppercase()); }

}