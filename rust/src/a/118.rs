use utils::read_str_lowercase;

fn main() {
    let vowels: &str = "aoyeui";
    let mut word: String = String::new();

    for letter in read_str_lowercase().chars() {
        if !vowels.contains(letter) {
            word.push('.');
            word.push(letter);
        }
    }
    println!("{}", word);
}
