use utils::read_vec;

fn main() {
    let mut numbers: Vec<String>= read_vec('+');

    if numbers.len() > 1 {
        numbers.sort();
        println!("{}", numbers.join("+"));
    }
    else {
        println!("{}", numbers[0]);
    }
}
