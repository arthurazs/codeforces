use utils::{read_str, read_vec};

fn main() {
    read_str();
    for value in read_vec::<char>(' ') {
        if value == '1' {
            println!("HARD");
            return;
        }
    }
    println!("EASY");
}