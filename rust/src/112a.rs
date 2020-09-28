mod utils;
use utils::read_str_lowercase;

fn main() {
    let mut first: String = read_str_lowercase();
    let mut second: String = read_str_lowercase();

    if first > second { println!("{}", 1); }
    else if first < second { println!("{}", -1); }
    else { println!("{}", 0); }
}
