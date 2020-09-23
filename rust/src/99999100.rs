fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn main() {
    println!("{}", read_str()
             .split_whitespace()
             .map(|number| number.parse::<u16>().expect("Error"))
             .sum::<u16>());
}
