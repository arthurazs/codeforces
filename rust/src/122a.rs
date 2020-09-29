mod utils;
use utils::read_int;

fn main() {
    let n: u16 = read_int();
    for &divisor in &[4, 7, 44, 47, 74, 77, 444, 447, 474, 477, 744, 747, 777] {
        if divisor > n { break; }
        if n % divisor == 0 {
            println!("YES");
            std::process::exit(0);
        }
    }
    println!("NO");
}