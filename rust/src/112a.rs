fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.make_ascii_lowercase();
    buffer.trim().to_string()
}

fn main() {
    let first: String = read_str();
    let second: String = read_str();

    if first > second { println!("{}", 1); }
    else if first < second { println!("{}", -1); }
    else { println!("{}", 0); }
}
