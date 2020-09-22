fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn read_int() -> u16 { read_str().parse().unwrap() }

fn main() {
    let mut total: u16 = 0;
    for _ in 0..read_int() {
        let sure_friends: u8 = read_str()
            .split_whitespace()
            .map(|friend| friend.parse::<u8>().expect("Error"))
            .sum();
        if sure_friends > 1 {
            total += 1;
        }
    }
    println!("{}", total);
}
