use utils::read_hash;

fn main() {
    let shoes: std::collections::HashSet<u32> = read_hash();
    print!("{}", 4 - shoes.len());
}