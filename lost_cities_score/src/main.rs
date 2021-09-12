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

use std::io::stdin;

fn main() {
    let player_1_score = 0_i16;
    let player_2_score = 0_i16;

    for round in 1..=3 {
        println!("round: {}", round);
        for user in 1..=2 {
            println!("  user: {}", user);
        }
    }

    println!("player 1 score: {}", player_1_score);
    println!("player 2 score: {}", player_2_score);
}
