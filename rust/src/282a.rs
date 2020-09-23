fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn read_int() -> u8 { read_str().parse().expect("Error") }

fn main() {
    let mut x: i16 = 0;
    for _ in 0..read_int() {
        let instruction: String = read_str();
        if &instruction[1..2] == "+" { x += 1; }
        else { x -= 1; }
    }
    println!("{}", x);
}
