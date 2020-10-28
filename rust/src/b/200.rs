use utils::{read_int, read_vec};

fn main() {
    let drinks: f32 = read_int();
    let volumes: Vec<f32> = read_vec(' ');
    print!("{}", volumes.iter().sum::<f32>() / drinks);
}