use utils::{read_str, read_vec, Utils};

fn main() {
    read_str();
    let (mut total, mut three, mut two, mut one): (u32, u32, u32, u32) = (0, 0, 0, 0);

    for number in read_vec::<u8>(' ').iter() {
        match number {
            4 => total += 1, 3 => three += 1,
            2 => two += 1, _ => one += 1
        }
    }

    total += three;
    one = one.saturating_sub(three);

    total += two / 2;
    if two % 2 == 1 {
        total += 1;
        one = one.saturating_sub(2);
    }

    total += one.div_ceil(4);

    println!("{}", total);
}
