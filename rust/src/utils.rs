use std::str::FromStr;

pub fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

pub fn read_str_lowercase() -> String {
    let mut buffer: String = read_str();
    buffer.to_lowercase()
}

pub fn read_int<T: FromStr>() -> T{
    read_str().parse::<T>().ok().expect("Error")
}

pub fn read_vec<T: FromStr>(split_char: char) -> Vec<T> {
    read_str()
        .split(split_char)
        .map(|value| value.parse::<T>().ok().expect("Error"))
        .collect()
}

pub fn div_ceil(dividend: u64, divisor: u64) -> u64 {
    (dividend + divisor - 1) / divisor
}