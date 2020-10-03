use utils::{read_vec, Utils};

fn main() {
    let (n, m, a): (u64, u64, u64) = {
        let values: Vec<u64> = read_vec(' ');
        (values[0], values[1], values[2])
    };

    println!("{}", n.div_ceil(a) * m.div_ceil(a));
}
