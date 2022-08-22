/** This file defines the structure of a state,
 * deals with its initialization
 * and menages it at the end of each turn.
 */

pub struct GameState {
    answer: String,
    guesses: Vec<String>,
}

pub struct State {
    total_rounds: u32,
    games: Vec<GameState>,
}

impl State {
    pub fn new() -> State {
        State {
            total_rounds: 0,
            games: Vec::new(),
        }
    }
}
