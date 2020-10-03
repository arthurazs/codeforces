use utils::{read_int, read_vec};

fn main() {
    let (mut x, mut y, mut z): (i8, i8, i8) = (0, 0, 0);
    let mut forces: Vec<i8>;

    for _ in 0..read_int() {
        forces = read_vec(' ');
        x += forces[0];
        y += forces[1];
        z += forces[2];
    }

    if x == 0 && y == 0 && z == 0 { println!("YES"); }
    else { println!("NO"); }
}
