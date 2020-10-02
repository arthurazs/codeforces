mod utils;
use utils::read_vec;

fn main() {
    let k: usize = read_vec::<u8>(' ')[1] as usize;
    let scores: Vec<u8> = read_vec(' ');

    let min: u8 = scores[k - 1];
    let mut total: u8 = 0;
    for score in scores {
        if score >= min && score > 0 {
            total += 1;
        }
        if score <= 0 || score < min {
            break;
        }
    }
    println!("{}", total);
}
