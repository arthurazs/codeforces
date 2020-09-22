fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn read_vec() -> Vec<u8> {
    read_str()
        .split_whitespace()
        .map(|value| value.parse::<u8>().expect("Error"))
        .collect()
}

fn main() {
    let k: usize = read_vec()[1] as usize;
    let scores: Vec<u8> = read_vec();

    let min: u8 = scores[k - 1];
    let mut total: u8 = 0;
    for score in scores {
        if score >= min && score > 0 {
            total += 1;
        }
        if score <= 0 || score < min {
            break;
        }
    }
    println!("{}", total);
}
