use crate::cli_parse::ConfigFormat;
use crate::state::State;
use std::collections::HashSet;
use std::hash::Hash;

// Processes the ConfigFormat passed in and returns a tuple
// consisting of the updated ConfigFormat, loaded or newly-created state, the answer, and HashSet of allowed guesses.
pub fn initialize(loaded_config: &mut ConfigFormat) -> (ConfigFormat, State, String, HashSet<String>){

}

// Called when both or one of the lists are custom-loaded.
// Checks if the answers_list is strictly a subset of the guesses_list.
fn check_guesses_and_answers(guesses_list: & HashSet<String>, answers_list: & HashSet<String>) -> bool {

}