fn read_vec() -> Vec<u16> {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer
        .split_whitespace()
        .map(|brother|
            brother.parse::<u16>().expect("Error"))
        .collect()
}

fn main() {
    let (mut a, mut b): (u16, u16) = {
        let brothers: Vec<u16> = read_vec();
        (brothers[0], brothers[1])
    };

    let mut years: u8 = 0;
    while a <= b {
        a *= 3;
        b *= 2;
        years += 1;
    }

    println!("{}", years);
}