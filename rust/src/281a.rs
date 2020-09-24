fn read_str() -> String {
    let mut buffer: String  = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn main() {
    let word: String = read_str();
    println!("{}{}", &word[0..1].to_uppercase(), &word[1..]);
}
