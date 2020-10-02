#[path = "../utils.rs"] mod utils;
use utils::read_str;

fn main() {
    let word: String = read_str();
    println!("{}{}", &word[0..1].to_uppercase(), &word[1..]);
}
