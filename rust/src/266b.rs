mod utils;
use utils::{read_str, read_vec};

fn main() {
    let times: u8 = read_vec(' ')[1];
    let mut queue: String = read_str();

    for _ in 0..times {
        let mut current: String = String::new();
        let mut flag: bool = false;
        let length: usize = queue.len();

        for pos in 0..length {
            if flag {
                flag = false;
                continue;
            }

            if pos + 2 <= length {
                if &queue[pos..pos + 2] == "BG" {
                    current.push_str("GB");
                    flag = true;
                    continue;
                }
            }
            current.push_str(&queue[pos..pos + 1]);
        }
        queue = current.clone();
    }
    println!("{}", queue);
}