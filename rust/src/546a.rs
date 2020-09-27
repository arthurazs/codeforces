fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn read_vec() -> Vec<u32> {
    read_str().trim()
        .split_whitespace()
        .map(|value| value
            .parse::<u32>()
            .expect("Error"))
        .collect()
}

fn main() {
    let (k, n, w): (u32, u32, u32) =  {
        let values: Vec<u32> = read_vec();
        (values[0], values[1], values[2])
    };

    let mut total: u32 = (1 + w) * w * k / 2;
    if total >= n { total -= n; }
    else { total = 0; }

    println!("{}", total);
}