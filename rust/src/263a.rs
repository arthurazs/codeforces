fn read_str() -> String {
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn read_vec() -> Vec<u8> {
    read_str()
        .split_whitespace()
        .map(|value| value.parse::<u8>().expect("Error"))
        .collect()
}

fn find(row: &Vec<u8>) -> (bool, u8) {
    for (index, &value) in row.iter().enumerate() {
        if value == 1 {
            if index != 2 {
                return (true, (index as i8 - 1).abs() as u8 % 2 + 1);
            }
            return (true, 0);
        }
    }
    (false, 0)
}

fn main() {
    let mut row: Vec<u8>;
    let mut found: bool = false;
    let mut moves: u8 = 0;

    for index in [2, 1, 0, 1, 2].iter() {
        row = read_vec();

        if !found {
            let new_value = find(&row);
            found = new_value.0;
            moves = index + new_value.1;
        }
    }

    println!("{}", moves);
}
