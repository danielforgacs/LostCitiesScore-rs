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
    for round in 1..=3 {
        println!("round: {}", round);
        for user in 1..=2 {
            println!("  user: {}", user);
        }
    }
}
