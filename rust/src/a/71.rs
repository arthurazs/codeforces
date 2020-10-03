use utils::{read_int, read_str};

fn main() {
    let test_length: i8 = read_int();
    
    for _ in 0..test_length {
        let mut word: String = read_str();
        let length: usize = word.len();
        if length > 10 {
             let mut new_word: String = String::new();
             new_word.push_str(&word[0..1]);
             new_word.push_str(&(length - 2).to_string());
             new_word.push_str(&word[length - 1..length]);
             word = new_word;
        }
        println!("{}", word);
    }
}
