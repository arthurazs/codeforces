use utils::read_vec;

fn main() {
    let (k, n, w): (u32, u32, u32) =  {
        let values: Vec<u32> = read_vec(' ');
        (values[0], values[1], values[2])
    };

    let mut total: u32 = (1 + w) * w * k / 2;
    if total >= n { total -= n; }
    else { total = 0; }

    println!("{}", total);
}