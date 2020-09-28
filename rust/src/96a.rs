mod utils;
use utils::read_str;

fn main() {
    let mut danger: u8 = 1;
    let players: String = read_str();
    let mut previous_player: char = players.chars().nth(0).expect("Error");

    for player in players[1..].chars() {
        if previous_player == player {
            danger += 1;
            if danger == 7 {
                println!("YES");
                std::process::exit(0);
            }
        }
        else { danger = 1; }
        previous_player = player;
    }
    println!("NO");
}