use std::str::FromStr;

pub fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

pub fn read_str_lowercase() -> String {
    let buffer: String = read_str();
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

pub fn read_tuple<T: FromStr + Copy>() -> (T, T) {
    let buffer: Vec<T> = read_vec(' ');
    (buffer[0], buffer[1])
}

pub trait Utils { fn div_ceil(self, divisor: Self) -> Self; }
macro_rules! impl_div { ($($t:ty)+) => { $(
    impl Utils for $t {
        fn div_ceil(self, divisor: Self) -> Self {
            (self + divisor - 1) / divisor
        }
    }
)*}}
impl_div!{u32 u64}

// use std::ops::{Add, Sub, Div};
// pub trait Parse { fn one() -> Self; }
// macro_rules! impl_one { ($($t:ty)+) => { $(impl Parse for $t { fn one() -> $t { 1 } })* } }
// impl_one!{u32 u64}
//
// pub fn div_ceil<T>(dividend: T, divisor: T) -> T
//     where T: Copy + Add<Output = T> + Parse + Sub<Output = T> + Div<Output = T> {
//     (dividend + divisor - Parse::one()) / divisor
// }