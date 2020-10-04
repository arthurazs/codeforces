use utils::read_vec;

fn main() {
    let h: u16 = read_vec(' ')[1];
    let mut width: u16 = 0;

    for friend in read_vec::<u16>(' ') {
        if friend > h { width += 1; }
        width += 1;
    }

    println!("{}", width);
}