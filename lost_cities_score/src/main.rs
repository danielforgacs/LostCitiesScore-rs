/*
game:
    round 1:
        player 1: "ddd23456789t dt"
        player 2: "d2 d3 d4 678 t"
    round 2:
        player 1: "ddd23456789t dt"
        player 2: "d2 d3 d4 678 t"
    round 3:
        player 1: "ddd23456789t dt"
        player 2: "d2 d3 d4 678 t"
*/

use std::io;

struct Player {
    score: i16,
}

impl Player {
    fn new() -> Self {
        Player{ score: 0 }
    }
}

fn main() {
    let mut players: [Player; 2] = [Player::new(), Player::new()];

    for round in 0..=2 {
        for player_number in 0..=1 {
            println!("round {}, player {} enter cards:", round+1, player_number+1);
            let mut line = String::from("ddd23456789t");
            // let mut line = String::new();
            // let result = io::stdin().read_line(&mut line);
            // match result {
            //     Ok(_) => {},
            //     Err(err) => {
            //         println!("Could not read player input!");
            //         println!("Error: {}", err);
            //         return
            //     }
            // }
            players[player_number].score += calc_round_score(line);
        }
    }

    for (index, player) in players.iter().enumerate() {
        println!("player {} score: {}", index+1, player.score);
    }
}

fn calc_round_score(cards_text: String) -> i16 {
    println!("{}", cards_text.trim());
    -20
}


#[test]
fn test_calc_round_score() {
    assert_eq!(calc_round_score("d".to_string()), -20);
}