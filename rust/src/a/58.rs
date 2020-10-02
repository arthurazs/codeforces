#[path = "../utils.rs"] mod utils;
use utils::read_str;

fn main() {
    let s: String = read_str();
    let mut found: usize = 0;
    let word: &str = "hello";

    for letter in s.chars() {
        if word.chars().nth(found).expect("Error") == letter {
            found += 1;
            if found == 5 {
                println!("YES");
                std::process::exit(0);
            }
        }
    }
    println!("NO");

}