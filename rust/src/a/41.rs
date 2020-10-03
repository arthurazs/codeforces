use utils::read_str;

fn main() {
    let (s, t): (String, String) = (read_str(), read_str());
    if s.chars().rev().collect::<String>() == t { println!("YES"); }
    else { println!("NO"); }
}
