mod utils;
use utils::read_vec;

fn main() {
    println!("{}", read_vec::<u16>(' ').iter().sum::<u16>());
}
