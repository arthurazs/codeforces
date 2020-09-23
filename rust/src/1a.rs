fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn main() {
    let n: u64;
    let m: u64;
    let a: u64;

    {
        let values: Vec<u64> = read_str()
            .split_whitespace()
            .map(|value| value.parse::<u64>().expect("Error"))
            .collect();
        n = values[0];
        m = values[1];
        a = values[2];
    }

    println!("{}", ((n + a - 1) / a) * ((m + a - 1) / a));
}
