fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn read_vec() -> Vec<u16> {
    read_str()
        .split_whitespace()
        .map(|value| value.parse::<u16>().expect("Error"))
        .collect()
}

fn main() {
    let m: u16;
    let n: u16;
    {
        let mn: Vec<u16> = read_vec();
        m = mn[0];
        n = mn[1];
    }
    println!("{}", (m * n) / 2);
}
