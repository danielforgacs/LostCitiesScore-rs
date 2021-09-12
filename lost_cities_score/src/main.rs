/*
round 1:
    player 1: ddd23456789t dt
    player 2: d2 d3 d4 678 t
round 2:
    player 1: ddd23456789t dt
    player 2: d2 d3 d4 678 t
round 3:
    player 1: ddd23456789t dt
    player 2: d2 d3 d4 678 t
*/

enum Card {
    double,
    number(u8),
    ten,
}

struct ExpeditionCards {
    cards: Vec<Card>,
}

struct RoundExpeditions {
    expeditions: Vec<ExpeditionCards>,
}

fn main() {
}
