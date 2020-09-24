fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn main() {
    read_str();
    let mut remove: u8 = 0;
    let stones: String = read_str();
    let mut current: char = stones.chars().nth(0).expect("Error");

    for stone in stones[1..].chars() {
        if current == stone { remove += 1; }
        current = stone;
    }
    println!("{}", remove)
}