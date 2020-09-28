mod utils;
use utils::{read_str, read_int};

fn main() {
    let mut x: i16 = 0;
    for _ in 0..read_int() {
        let instruction: String = read_str();
        if &instruction[1..2] == "+" { x += 1; }
        else { x -= 1; }
    }
    println!("{}", x);
}
