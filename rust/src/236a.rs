use std::collections::HashSet;

fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn main() {
    let mut letters: HashSet<char> = HashSet::new();

    for letter in read_str().chars() {
        letters.insert(letter);
    }

    if letters.len() % 2 == 0 { println!("CHAT WITH HER!"); }
    else { println!("IGNORE HIM!"); }

}