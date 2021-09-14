use chrono::{DateTime, Utc};
use std::io::{self, Write};

const GAME_LOG_FILE_NAME: &str = "LostCitiesScores";
const GAME_LOG_DATE_TEMPLATE: &str = "%Y-%m-%d_%H-%M-%S";

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
    let logname = create_game_log_name();

    println!(
        "===== Lost Cities Scores Counter =====\n\
        type 'quit' to finish the game. \
        \ncards can be: d=double, t=10, 2-9\n\
        game log name:\n{}\n\
        ========================================\n",
        logname
    );

    let mut log = String::new();
    let mut players: [Player; 2] = [Player::new(), Player::new()];

    for round in 0..=2 {
        for player_number in 0..=1 {
            players[player_number].score = loop {
                print!("--> Enter round: {}, player {} cards: ", round + 1, player_number + 1);
                io::stdout().flush().unwrap();

                let mut line = String::new();
                let stdin_result = io::stdin().read_line(&mut line);

                match stdin_result {
                    Err(err) => {
                        println!("Could not read player input! Error: {}", err);
                        return;
                    },
                    _ => {},
                }

                match line.as_str() {
                    "quit\n" => {
                        println!("Bye!");
                        return;
                    },
                    "\n" => {
                        println!("Player must have a card!");
                        continue;
                    },
                    _ => {},
                };

                
                match calc_player_round_score(&line) {
                    Ok(score) => {
                        let logline = format!("round: {}, player {} cards: {}", round + 1, player_number + 1, line);
                        log += &logline.as_str();
                        break score
                    },
                    Err(Error::CardError(card)) => { println!("Bad card: \"{:?}\"!", card); },
                };
            };
        }
    }

    for (index, player) in players.iter().enumerate() {
        log += &format!("player {} score: {}\n", index + 1, player.score).as_str();
    }

    println!("\n\nResults - [log:{}]:", logname);
    println!("{}", log);

    match std::fs::write(logname, log) {
        Ok(_) => {}
        Err(_) => { println!("Could not save log file sof some reason."); }
    };
}

fn calc_player_round_score(line: &String) -> Result<i16, Error> {
    let mut round_score = 0;

    for colour in line.split(' ') {
        let colour = colour.trim();

        match calc_expedition_score(&colour.to_string()) {
            Ok(score) => round_score += score,
            Err(Error::CardError(card)) => { return Result::Err(Error::CardError(card)); }
        };
    }

    Result::Ok(round_score)
}

fn create_game_log_name() -> String {
    loop {
        let now: DateTime<Utc> = Utc::now();
        let logname = format!("{}_{}.txt", GAME_LOG_FILE_NAME, now.format(GAME_LOG_DATE_TEMPLATE));

        if !std::path::Path::new(&logname).exists() {
            return logname;
        };
    }
}

fn calc_expedition_score(cards_text: &String) -> Result<i16, Error> {
    let mut score = 0_i16;
    let mut doubler = 0_u8;

    for card in cards_text.trim().chars() {
        match card {
            'd' => doubler += 1,
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { score += card.to_digit(10).unwrap() as i16 }
            't' | '1' => score += 10,
            _ => return Result::Err(Error::CardError(card)),
        };
    }

    score = (score - 20) * (doubler as i16 + 1);

    if cards_text.len() >= 8 {
        score += 20;
    };

    Result::Ok(score)
}

#[test]
fn test_calc_expedition_score() {
    assert_eq!(
        calc_expedition_score(&"5".to_string()).unwrap(),
        -15
    );
    assert_eq!(
        match calc_expedition_score(&"d5".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -30
    );
    assert_eq!(
        match calc_expedition_score(&"dd5".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -45
    );
    assert_eq!(
        match calc_expedition_score(&"d".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -40
    );
    assert_eq!(
        match calc_expedition_score(&"dd".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -60
    );
    assert_eq!(
        match calc_expedition_score(&"ddd".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -80
    );
    assert_eq!(
        match calc_expedition_score(&"2345678".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        2 + 3 + 4 + 5 + 6 + 7 + 8 - 20
    );
    assert_eq!(
        match calc_expedition_score(&"23456789".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        44 - 20 + 20
    );
    assert_eq!(
        match calc_expedition_score(&"23456789t".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 - 20 + 20
    );
    assert_eq!(
        match calc_expedition_score(&"7891".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        14
    );
    assert_eq!(
        match calc_expedition_score(&"d7891".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        14 * 2
    );
    assert_eq!(
        match calc_expedition_score(&"dd7891".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        14 * 3
    );
    assert_eq!(
        match calc_expedition_score(&"dd789t".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        14 * 3
    );
    assert_eq!(
        match calc_expedition_score(&"2".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -18
    );
    assert_eq!(
        match calc_expedition_score(&"23".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -20 + 2 + 3
    );
    assert_eq!(
        match calc_expedition_score(&"234".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -20 + 2 + 3 + 4
    );
    assert_eq!(
        match calc_expedition_score(&"23456789".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 20
    );
    assert_eq!(
        match calc_expedition_score(&"2345678".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        -20 + 2 + 3 + 4 + 5 + 6 + 7 + 8
    );
    assert_eq!(
        match calc_expedition_score(&"d23456789".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 2) + 20
    );
    assert_eq!(
        match calc_expedition_score(&"dd23456789".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 3) + 20
    );
    assert_eq!(
        match calc_expedition_score(&"ddd23456789".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 4) + 20
    );
    assert_eq!(
        match calc_expedition_score(&"ddd2345678".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8) * 4) + 20
    );
    assert_eq!(
        match calc_expedition_score(&"ddd23456".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        ((-20 + 2 + 3 + 4 + 5 + 6) * 4) + 20
    );
    assert_eq!(
        match calc_expedition_score(&"ddd2345".to_string()) {
            Ok(k) => k,
            Err(_) => -999,
        },
        ((-20 + 2 + 3 + 4 + 5) * 4)
    );


    assert_eq!(calc_expedition_score(&"ddd2345".to_string()).unwrap(), (-20 + 2 + 3 + 4 + 5) * 4);
}

#[test]
fn test_calc_player_round_score() {
    assert_eq!(calc_player_round_score(&"28t 28t".to_string()).unwrap(), 0);
    assert_eq!(calc_player_round_score(&"d d d d d".to_string()).unwrap(), -200);
    assert_eq!(calc_player_round_score(&"dd dd dd dd dd".to_string()).unwrap(), -300);
    assert_eq!(calc_player_round_score(&"ddd d ddd d ddd".to_string()).unwrap(), -320);
    assert_eq!(calc_player_round_score(&"2 d34 dd456 ddd5678 ddd23456789t".to_string()).unwrap(), 121);
    assert_eq!(calc_player_round_score(&"ddd23456789t".to_string()).unwrap(), 156);
    assert_eq!(calc_player_round_score(&"ddd23456789t ddd23456789t ddd23456789t ddd23456789t ddd23456789t".to_string()).unwrap(), 780);
}