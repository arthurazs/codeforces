fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

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
