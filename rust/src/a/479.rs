use utils::read_int;

fn main() {
    let (a, b, c): (u16, u16, u16) = (read_int(), read_int(), read_int());

    if a + c == 2 { println!("{}", a + b + c); }
    else if a == 1 || b == 1 || c == 1 {
        if a < c { println!("{}", (a + b) * c); }
        else { println!("{}", a * (b + c)); }
    }
    else { println!("{}", a * b * c); }
}