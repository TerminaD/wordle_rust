use std::{fs::File, io::BufReader, path::Path};

/** This file deals with the state json file.
 *
 * TODO: everything
 */
use serde::{Deserialize, Serialize};

use crate::{internal_state::AttemptState, logic::{is_five_letter_capitalized, check_valid_entry}};

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

pub fn json_stat_append(att: &AttemptState, ans: &String) {}

pub fn update_from_json_stat(stat_path: Path, answer_list: &Vec<String>, guess_list: &Vec<String>) -> (AttemptState, String) {
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
	let answer_char:[char; 5] = answer.chars().collect();

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

fn check_valid_json_stat() {}

pub fn calculate_stats() {}

mod tests {}
