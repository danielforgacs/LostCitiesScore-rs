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
        let logline = format!("Round {}:\n", round+1);
        print!("{}", logline);
        log += logline.as_str();

        for player_number in 0..=1 {
            players[player_number].score += loop {
                let logline = format!("   player {} cards: ", player_number + 1);
                print!("{}", logline);

                io::stdout().flush().unwrap();

                let mut user_input = String::new();
                let stdin_result = io::stdin().read_line(&mut user_input);

                match stdin_result {
                    Err(err) => {
                        println!("Could not read player input! Error: {}", err);
                        return;
                    },
                    _ => {},
                }

                match user_input.as_str() {
                    "quit\n" => {
                        println!("Bye!");
                        return;
                    },
                    _ => {},
                };

                match sanity_check_player_cards(&user_input) {
                    true => {}
                    false => { continue }
                }

                log += logline.as_str();

                match calc_player_round_score(&user_input) {
                    Ok(score) => {
                        let logline = format!("{}", user_input);
                        log += &logline.as_str();
                        break score
                    },
                    Err(Error::CardError(card)) => { println!("Bad card: \"{:?}\"!", card); },
                };
            };
        }
    }

    let mut winner_index = 0;

    if players[0].score < players[1].score {
        winner_index = 1;
    };

    for (index, player) in players.iter().enumerate() {
        log += &format!("player {} score: {}", index + 1, player.score).as_str();

        if index == winner_index {
            log += &format!(" <-- WINNER\n");
        } else {
            log += &format!("\n");
        };
    }

    println!("\n\nResults - log file: {}:", logname);
    println!("{}", log);

    match std::fs::write(logname, log) {
        Ok(_) => {}
        Err(_) => { println!("Could not save log file sof some reason."); }
    };
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

fn sanity_check_player_cards(user_input: &str) -> bool {
    let mut is_input_valid = true;
    let mut expedition_count = 0;

    match user_input {
        "\n" | "" => {
            println!("User must have cards!");
            return false;
        },
        _ => {},
    };

    for expedition in user_input.split(' ') {
        expedition_count += 1;
        
        if expedition_count > 6 {
            println!("Too many expeditions. Count: {}, Cards: {}", expedition_count, user_input);
            is_input_valid = false;
            break
        };
        
        let mut valid_cards = vec!['d', 'd', 'd', '2', '3', '4', '5', '6', '7', '8', '9', 't', '\n'];

        'card_loop: for card in expedition.chars() {

            for index in 0..valid_cards.len() {
                if card == valid_cards[index] {
                    valid_cards.remove(index);
                    continue 'card_loop;
                };
            };

            println!("Bad or duplicate card: \"{}\"", card);
            is_input_valid = false;
            break;
        };
    };

    is_input_valid
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
    assert_eq!(calc_expedition_score(&"5".to_string()).unwrap(), -15);
    assert_eq!(calc_expedition_score(&"d5".to_string()).unwrap(), -30);
    assert_eq!(calc_expedition_score(&"dd5".to_string()).unwrap(), -45);
    assert_eq!(calc_expedition_score(&"d".to_string()).unwrap(), -40);
    assert_eq!(calc_expedition_score(&"dd".to_string()).unwrap(), -60);
    assert_eq!(calc_expedition_score(&"ddd".to_string()).unwrap(), -80);
    assert_eq!(calc_expedition_score(&"2345678".to_string()).unwrap(), 2 + 3 + 4 + 5 + 6 + 7 + 8 - 20);
    assert_eq!(calc_expedition_score(&"23456789".to_string()).unwrap(), 44 - 20 + 20);
    assert_eq!(calc_expedition_score(&"23456789t".to_string()).unwrap(), 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 - 20 + 20);
    assert_eq!(calc_expedition_score(&"7891".to_string()).unwrap(), 14);
    assert_eq!(calc_expedition_score(&"d7891".to_string()).unwrap(), 14 * 2);
    assert_eq!(calc_expedition_score(&"dd7891".to_string()).unwrap(), 14 * 3);
    assert_eq!(calc_expedition_score(&"dd789t".to_string()).unwrap(), 14 * 3);
    assert_eq!(calc_expedition_score(&"2".to_string()).unwrap(), -18);
    assert_eq!(calc_expedition_score(&"23".to_string()).unwrap(), -20 + 2 + 3);
    assert_eq!(calc_expedition_score(&"234".to_string()).unwrap(), -20 + 2 + 3 + 4);
    assert_eq!(calc_expedition_score(&"23456789".to_string()).unwrap(), -20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 20);
    assert_eq!(calc_expedition_score(&"2345678".to_string()).unwrap(), -20 + 2 + 3 + 4 + 5 + 6 + 7 + 8);
    assert_eq!(calc_expedition_score(&"d23456789".to_string()).unwrap(), ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 2) + 20);
    assert_eq!(calc_expedition_score(&"dd23456789".to_string()).unwrap(), ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 3) + 20);
    assert_eq!(calc_expedition_score(&"ddd23456789".to_string()).unwrap(), ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9) * 4) + 20);
    assert_eq!(calc_expedition_score(&"ddd2345678".to_string()).unwrap(), ((-20 + 2 + 3 + 4 + 5 + 6 + 7 + 8) * 4) + 20);
    assert_eq!(calc_expedition_score(&"ddd23456".to_string()).unwrap(), ((-20 + 2 + 3 + 4 + 5 + 6) * 4) + 20);
    assert_eq!(calc_expedition_score(&"ddd2345".to_string()).unwrap(), ((-20 + 2 + 3 + 4 + 5) * 4));
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
    assert_eq!(calc_player_round_score(&"45789t dd3458t d23478t".to_string()).unwrap(), 81);
    assert_eq!(calc_player_round_score(&"d234689 d23569t 69 dd56789t".to_string()).unwrap(), 144);
}

#[test]
fn test_sanity_check_player_cards() {
    assert_eq!(sanity_check_player_cards(&"2 3 4 5 6 7 8 9"), false);
    assert_eq!(sanity_check_player_cards(&"dddd"), false);
    assert_eq!(sanity_check_player_cards(&"23x"), false);
    assert_eq!(sanity_check_player_cards(&"2 2 2 2s"), false);
    assert_eq!(sanity_check_player_cards(&"2 2 2 2"), true);
    assert_eq!(sanity_check_player_cards(&""), false);
    assert_eq!(sanity_check_player_cards(&"d23 dd345 ddd45678 ddd 23456y"), false);
    assert_eq!(sanity_check_player_cards(&"dddd"), false);
    assert_eq!(sanity_check_player_cards(&"dd23456789t dd23456789t"), true);
    assert_eq!(sanity_check_player_cards(&"t98765432ddd t98765432ddd"), true);
    assert_eq!(sanity_check_player_cards(&"t98765432dddt98765432ddd"), false);
    assert_eq!(sanity_check_player_cards(&"t98765432ddd t987654932ddd"), false);
}