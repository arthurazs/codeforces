use utils::read_str;

fn main() {
    let (length, a): (usize, u128) = {
        let s: String = read_str();
        (s.len(), u128::from_str_radix(&s[..], 2).expect("Error"))
    };
    let b: u128 = {
        let s: String = read_str();
        u128::from_str_radix(&s[..], 2).expect("Error")
    };

    print!("{:01$b}", a ^ b, length);
}