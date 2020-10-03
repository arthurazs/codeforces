use utils::read_int;

fn main() {
    let mut year: u16 = read_int();

    loop {
        year += 1;
        let hash: std::collections::HashSet<char> = year.to_string().chars().collect();
        if hash.len() == 4 { println!("{}", year); break; }
    }
}