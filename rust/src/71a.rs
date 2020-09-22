use std::io::stdin;


fn read_int() -> i8 {
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().parse::<i8>().unwrap()
}

fn read_str() -> String {
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("Error");
    buffer.trim().to_string()
}

fn main() {
    let test_length: i8 = read_int();
    
    for _ in 0..test_length {
        let mut word: String = read_str();
        let length: usize = word.len();
        if length > 10 {
             let mut new_word: String = String::new();
             new_word.push(word.chars().nth(0).unwrap());
             new_word.push_str(&(length - 2).to_string());
             new_word.push(word.chars().nth(length - 1).unwrap());
             word = new_word;
        }
        println!("{}", word);
    }
}
