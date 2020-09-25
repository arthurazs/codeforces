fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn read_vec() -> Vec<i8> {
    read_str()
        .split_whitespace()
        .map(|force| force
             .parse::<i8>()
             .expect("Error"))
        .collect()
}

fn main() {
    let mut x: i8 = 0;
    let mut y: i8 = 0;
    let mut z: i8 = 0;
    let mut forces: Vec<i8>; 

    for _ in 0..read_str().parse::<u8>().expect("Error") {
        forces = read_vec();
        x += forces[0];
        y += forces[1];
        z += forces[2];
    }

    if x == 0 && y == 0 && z == 0 { println!("YES"); }
    else { println!("NO"); }
}
