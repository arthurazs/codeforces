use utils::read_str_lowercase;

fn main() {
    let first: String = read_str_lowercase();
    let second: String = read_str_lowercase();

    if first > second { println!("{}", 1); }
    else if first < second { println!("{}", -1); }
    else { println!("{}", 0); }
}
