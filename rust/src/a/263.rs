use utils::read_vec;

fn find(row: &Vec<u8>) -> (bool, i8) {
    for (index, &value) in row.iter().enumerate() {
        if value == 1 {
            return (true, (index as i8 - 2).abs());
        }
    }
    (false, 0)
}

fn main() {
    let mut row: Vec<u8>;
    let mut found: bool = false;
    let mut moves: i8 = 0;

    for index in [2, 1, 0, 1, 2].iter() {
        row = read_vec(' ');

        if !found {
            let new_value = find(&row);
            found = new_value.0;
            moves = index + new_value.1;
        }
    }

    println!("{}", moves);
}
