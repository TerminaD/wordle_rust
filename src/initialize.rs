use crate::cli_parse::ConfigFormat;
use crate::state::State;
use std::collections::BTreeSet;

// Processes the ConfigFormat passed in and returns a tuple
// consisting of the updated ConfigFormat, loaded or newly-created state, the answer, and BTreeSet of allowed guesses.
pub fn initialize(loaded_config: &mut ConfigFormat)
-> (State, String, BTreeSet<String>) {
    match loaded_config.config {
        Some(config_file_patch) => {
            // TODO: Load config from JSON file
        }
        None => {}
    }

    let mut custom_list_flag = false;
    let mut guesses_list: BTreeSet<String> = BTreeSet::new();
    let mut answers_list: BTreeSet<String> = BTreeSet::new();
    // TODO: Load answer_list and guess_list, set flag
    if custom_list_flag && !(answers_list.is_subset(&guesses_list) && answers_list != guesses_list) {
        panic!("List of answers is not a strict subset of the list of guesses.")
    }

    let answer: String;
    if loaded_config.random {
        // TODO: Unwrap day and seed, and generate a random answer.
    }
    else {
        // TODO: fix cli_parse to read a designated answer.
    }

    let mut curr_state: State;
    // TODO: create new or load state.
    
    (curr_state, answer, guesses_list)
}