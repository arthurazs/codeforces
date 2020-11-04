use utils::{read_int, read_vec};

fn main() {
    let length: usize = read_int::<usize>() - 1;
    let line: Vec<u8> = read_vec(' ');
    let (mut pos_min, mut pos_max): (usize, usize) = (0, 0);
    let (mut height_min, mut height_max): (u8, u8) = (u8::max_value(), 0);

    for (index, &height) in line.iter().enumerate() {
        if height <= height_min { pos_min = index; height_min = height; }
        if height > height_max { pos_max = index; height_max = height; }
    }
    print!("{}", (pos_max + length - pos_min) - if pos_max > pos_min { 1 } else { 0 });
}