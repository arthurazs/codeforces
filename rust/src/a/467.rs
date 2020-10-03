use utils::{read_int, read_tuple};

fn main() {
    let mut free_rooms: u8 = 0;
    for _ in 0..read_int::<u8>() {
        let (p, q): (u8, u8) = read_tuple();
        if q - p >= 2 { free_rooms += 1; }
    }
    println!("{}", free_rooms);
}