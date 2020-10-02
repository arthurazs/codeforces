#[path = "../utils.rs"] mod utils;
use utils::read_str;

fn main() {
    let vowels: &str = "aoyeui";
    let mut word: String = String::new();

    for letter in read_str().to_lowercase().chars() {
        if !vowels.contains(letter) {
            word.push('.');
            word.push(letter);
        }
    }
    println!("{}", word);
}
