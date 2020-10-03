use utils::read_vec;

fn main() {
    let m: u16;
    let n: u16;
    {
        let mn: Vec<u16> = read_vec(' ');
        m = mn[0];
        n = mn[1];
    }
    println!("{}", (m * n) / 2);
}
