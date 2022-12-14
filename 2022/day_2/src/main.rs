use std::fs::File;
use std::io::{BufRead, BufReader};

const LOSE : i8 = -1;
const DRAW: i8 = 0;
const WIN: i8 = 1;


fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        println!("playing hand: {}", line);

        let round: Vec<&str> = line.split(' ').collect();

        let opponent_hand = round[0];
        let player_hand = round[1];
        let outcome = calculate_outcome(opponent_hand, player_hand);

        match outcome {
            LOSE => println!("YOU LOST!"),
            DRAW => println!("YOU DREW!"),
            WIN => println!("YOU WON!"),
            _ => println!("unknown outcome")
        }

        let player_score = calculate_player_score(player_hand);
        let round_score = calculate_round_score(outcome);

        let total_score = player_score + round_score;
        total += total_score as u32;
    }

    println!("The total score is: {}", total);
}


fn calculate_round_score(outcome: i8) -> u8 {
    return match outcome {
        LOSE => 0,
        DRAW => 3,
        WIN => 6,
        _ => panic!("unknown outcome")
    }
}

fn calculate_player_score(hand: &str) -> u8 {
    return match hand {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("unknown hand")
    }
}

fn calculate_outcome(opponent: &str, player: &str) -> i8 {
    return match opponent {
        "A" => match player {
            "X" => DRAW,
            "Y" => WIN,
            "Z" => LOSE,
            _ => panic!("unknown player hand")
        },
        "B" => match player {
            "X" => LOSE,
            "Y" => DRAW,
            "Z" => WIN,
            _ => panic!("unknown player hand")
        },
        "C" => match player {
            "X" => WIN,
            "Y" => LOSE,
            "Z" => DRAW,
            _ => panic!("unknown player hand")
        },
        _ => panic!("unknown opponent hand")
    }
}