use utils::{read_str, read_int};

fn main() {
    let mut previous: String = String::new();
    let mut groups: u32 = 0;

    for _ in 0..read_int::<u32>() {
        let magnet: String = read_str();
        if magnet != previous {
            groups += 1;
            previous = magnet;
        }
    }

    println!("{}", groups);
}