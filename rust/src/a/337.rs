use utils::read_vec;

fn main() {
    let students: usize = read_vec(' ')[0];
    let mut pieces: Vec<u16> = read_vec(' ');
    pieces.sort();
    let mut difference: u16 = u16::max_value();

    let mut index: usize = 0;
    let end: usize = pieces.len() - students;
    loop {
        difference = std::cmp::min(difference, pieces[index + students - 1] - pieces[index]);
        index += 1;
        if index > end { break; }
    }
    print!("{}", difference);
}