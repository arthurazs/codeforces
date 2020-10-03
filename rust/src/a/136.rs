use utils::{read_int, read_vec};

fn main() {
    let mut friends: Vec<String> = vec![String::new(); read_int()];
    for (index, &value) in read_vec::<u8>(' ').iter().enumerate() {
        friends[(value - 1) as usize] = (index + 1).to_string();
    }
    println!("{}", friends.join(" "));
}