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
#[derive(Debug)]
enum Error {
    CardError(char),
}

impl Player {
    fn new() -> Self {
        Player { score: 0 }
    }
}

fn main() {
    let mut players: [Player; 2] = [Player::new(), Player::new()];

    for round in 0..=2 {
        for player_number in 0..=1 {
            println!(
                "round {}, player {} enter cards:",
                round + 1,
                player_number + 1
            );
            // let line = String::from("ddd23456789t");
            // let line = String::from("5");
            // let line = String::from("23456789");
            let line = loop {
                let mut line = String::new();
                let result = io::stdin().read_line(&mut line);
                match result {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Could not read player input!");
                        println!("Error: {}", err);
                        return
                    }
                }
                if line != "\n" {
                    break line
                }

                println!("Enter player cards...")
            };
            let current_score = match calc_round_score(line) {
                Ok(score) => score,
                Err(Error::CardError(card)) => {
                    println!("Bad card: \"{}\"! Cards can be d, t, 2-9", card);
                    return
                }
            };
            players[player_number].score += current_score;
        }
    }

    for (index, player) in players.iter().enumerate() {
        println!("player {} score: {}", index + 1, player.score);
    }
}

fn calc_round_score(cards_text: String) -> Result<i16, Error> {
    let mut score = 0_i16;
    let mut doubler = 0_u8;

    for card in cards_text.trim().chars() {
        match card {
            'd' => doubler += 1,
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                score += card.to_digit(10).unwrap() as i16
            }
            't' | '1' => score += 10,
            _ => { return Result::Err(Error::CardError(card)) },
        };
    }

    score = (score - 20) * (doubler as i16 + 1);

    if cards_text.len() >= 8 {
        score += 20;
    };

    Result::Ok(score)
}

#[test]
fn test_calc_round_score() {
    assert_eq!(match calc_round_score("5".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -15);
    assert_eq!(match calc_round_score("d5".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -30);
    assert_eq!(match calc_round_score("dd5".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -45);
    assert_eq!(match calc_round_score("d".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -40);
    assert_eq!(match calc_round_score("dd".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -60);
    assert_eq!(match calc_round_score("ddd".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -80);
    assert_eq!(match calc_round_score("2345678".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, 2 + 3 + 4 + 5 + 6 + 7 + 8 - 20);
    assert_eq!(match calc_round_score("23456789".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, 44 - 20 + 20);
    assert_eq!(match calc_round_score("23456789t".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 - 20 + 20);
    assert_eq!(match calc_round_score("7891".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, 14);
    assert_eq!(match calc_round_score("d7891".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, 14 * 2);
    assert_eq!(match calc_round_score("dd7891".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, 14 * 3);
    assert_eq!(match calc_round_score("dd789t".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, 14 * 3);
    assert_eq!(match calc_round_score("2".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -18);
    assert_eq!(match calc_round_score("23".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -20 + 2 + 3);
    assert_eq!(match calc_round_score("234".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -20 + 2 + 3 + 4);
    assert_eq!(match calc_round_score("23456789".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 20);
    assert_eq!(match calc_round_score("2345678".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, -20 + 2 + 3 + 4 + 5 + 6 + 7 + 8);
    assert_eq!(match calc_round_score("d23456789".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 2) + 20);
    assert_eq!(match calc_round_score("dd23456789".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 3) + 20);
    assert_eq!(match calc_round_score("ddd23456789".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 4) + 20);
    assert_eq!(match calc_round_score("ddd2345678".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8) * 4) + 20);
    assert_eq!(match calc_round_score("ddd23456".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, ((-20 + 2 + 3 + 4 + 5 + 6) * 4) + 20);
    assert_eq!(match calc_round_score("ddd2345".to_string()) {
        Ok(k) => k,
        Err(_) => -999,
    }, ((-20 + 2 + 3 + 4 + 5) * 4));
}
