use utils::read_str;

fn main() {
    let word: String = read_str();
    let rest: &str = &word[1..];

    if rest == rest.to_uppercase() {
        let first: char = word.chars().nth(0).expect("Error");
        match first.is_uppercase() {
            true => println!("{}{}", first.to_ascii_lowercase(), rest.to_lowercase()),
            false => println!("{}{}", first.to_ascii_uppercase(), rest.to_lowercase())
        }
        return;
    }
    println!("{}", word);
}