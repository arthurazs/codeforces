use std::io::stdin;

fn read_int() -> i8 {
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().parse::<i8>().unwrap()
}

fn main() {
    let value: i8 = read_int();
    if value % 2 == 0 && value != 2 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}
