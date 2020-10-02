#[path = "../utils.rs"] mod utils;
use utils::read_str;

fn main() {
    for instruction in read_str().chars() {
        if "HQ9".contains(instruction) {
            println!("YES");
            std::process::exit(0);
        }
    }
    println!("NO");
}