use utils::read_tuple;

fn main() {
    let (mut a, mut b): (u16, u16) = read_tuple();

    let mut years: u8 = 0;
    while a <= b {
        a *= 3;
        b *= 2;
        years += 1;
    }

    println!("{}", years);
}