/** This file deals with the state json file.
 */

use serde::{Deserialize, Serialize};
use serde_json::to_writer;

use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use crate::{
    internal_state::AttemptState,
    logic::{check_valid_entry, is_five_letter_capitalized},
};

#[derive(Serialize, Deserialize)]
struct GameFormat {
    answer: String,
    guesses: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct StatFormat {
    total_rounds: u32,
    games: Vec<GameFormat>,
}

pub fn json_stat_append(stat_path: Path, atts: &AttemptState, ans: &String) {
    let mut new_guesses: Vec<String>;
    let mut new_word: String;
    for att in atts.attempts {
        new_word.clear();
        for i in 0..5 {
            new_word.push(att[i].letter);
        }
        new_guesses.push(new_word);
    }

    let new_game = GameFormat {
        answer: ans,
        guesses: new_guesses,
    };

    let file = File::open(stat_path).expect("Failed to open stat json file.");
    let reader = BufReader::new(file);
    let mut stat: StatFormat =
        serde_json::from_reader(reader).expect("Failed to convert json file to Rust object.");

    stat.total_rounds += 1;
    stat.games.push(new_game);

    let file_ref = &file;
    to_writer(file_ref, &stat);
}

pub fn update_from_json_stat(
    stat_path: Path,
    answer_list: &Vec<String>,
    guess_list: &Vec<String>,
) -> (AttemptState, String) {
    let file = File::open(stat_path).expect("Failed to open stat json file.");
    let reader = BufReader::new(file);
    let stat: StatFormat =
        serde_json::from_reader(reader).expect("Failed to convert json file to Rust object.");

    let answer = stat
        .games
        .last()
        .expect("The specified json file contains no game information.")
        .answer;
    if !check_valid_entry(&answer, &answer_list) {
        panic!("The answer in stat json file is not an all capitalized 5 letter word included in the list of candidate answers.");
    }
    let answer_char: [char; 5] = answer.chars().collect();

    let prev_attempts = stat
        .games
        .last()
        .expect("The specified json file contains no game information.")
        .guesses;
    for i in 0..prev_attempts.len() {
        if !check_valid_entry(&prev_attempts[i], &guess_list) {
            panic!("The guesses in stat json file is not an all capitalized 5 letter word included in the list of candidate guesses.");
        }
    }

    let mut attempts = AttemptState::new();
    for i in 0..prev_attempts.len() {
        attempts.push_attempt(&prev_attempts[i], &answer_char);
    }

    return (attempts, answer);
}

pub fn calculate_stats(stat_path: Path) {
    let file = File::open(stat_path).expect("Failed to open stat json file.");
    let reader = BufReader::new(file);
    let stat: StatFormat =
        serde_json::from_reader(reader).expect("Failed to convert json file to Rust object.");

    let wins: u32;
    let total_games: u32;
    let total_attempt: u32;
    let mut word_freq: HashMap<String, u32> = HashMap::new();

    total_games = stat.total_rounds;

    for game in stat.games {
        if game.guesses.len() < 6 {
            wins += 1;
        } else if game.guesses.len() == 6 && game.guesses.last().unwrap() == game.answer {
            wins += 1;
        }

        total_attempt += game.guesses.len();

        for att in game.guesses {
            word_freq[att] += 1;
        }
    }

    let mut best_word: String;
    let mut best_num: u32 = 0;

    for i in 0..5 {
        if word_freq.is_empty() {
            break;
        }

        for (word, appear) in word_freq.iter() {
            if appear > &best_num {
                best_word = word;
                best_num = appear;
            }
        }

        print!("The word {} is used {} times.", best_word, best_num);
        word_freq.remove(&best_word);
        best_num = 0;
    }

    print!(
        "Won {} games, lost {} games, averaged {} attempts per round.",
        wins,
        total_games - wins,
        total_attempt as f32 / wins as f32
    );
}
