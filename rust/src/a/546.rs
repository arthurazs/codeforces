use utils::read_vec;

fn main() {
    let (k, n, w): (u32, u32, u32) =  {
        let values: Vec<u32> = read_vec(' ');
        (values[0], values[1], values[2])
    };

    println!("{}", ((1 + w) * w * k / 2).saturating_sub(n));
}