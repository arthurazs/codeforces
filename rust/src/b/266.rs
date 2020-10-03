use utils::{read_str, read_vec};

fn main() {
    let times: u8 = read_vec(' ')[1];
    let mut queue: String = read_str();
    for _ in 0..times { queue = queue.replace("BG", "GB"); }
    println!("{}", queue);
}