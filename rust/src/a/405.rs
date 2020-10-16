use utils::{read_str, read_vec};

fn main() {
    read_str();
    let mut blocks: Vec<u8> = read_vec(' ');
    blocks.sort();
    for number in blocks { print!("{} ", number); }
}