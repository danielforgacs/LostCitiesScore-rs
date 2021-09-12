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
    let players: [Player; 2] = [Player::new(), Player::new()];

    for round in 1..=3 {
        for user in 1..=2 {
            println!("round {}, user {} cards:", round, user);
            let mut line = String::new();
            let result = io::stdin().read_line(&mut line);
        }
    }

    println!("player 1 score: {}", players[0].score);
    println!("player 2 score: {}", players[1].score);
}
