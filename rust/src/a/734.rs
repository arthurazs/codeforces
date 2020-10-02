#[path = "../utils.rs"] mod utils;
use utils::read_str;

fn main() {
    read_str();
    let mut score: i32 = 0;

    for winner in read_str().chars() {
        if winner == 'A' { score += 1; }
        else { score -= 1; }
    }

    println!("{}", if score > 0 { "Anton" } else if score < 0 { "Danik" } else { "Friendship" });
}