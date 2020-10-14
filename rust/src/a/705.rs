use utils::read_int;

fn main() {
    print!("I hate ");
    for index in 1..read_int::<u8>() {
        print!("that ");
        if index % 2 == 1 { print!("I love "); }
        else { print!("I hate "); }
    }
    println!("it")
}