#[path = "../utils.rs"] mod utils;
use utils::read_int;

fn main() {
    let value: i8 = read_int();
    if value % 2 == 0 && value != 2 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}
