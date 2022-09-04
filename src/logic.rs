/** This file deals with logic involving
 * answer generating, checking for an attempt's validity, and determining when the game ends.
 */

use std::io;

use crate::internal_state::{AttemptState, Block, AlphabetState, LetterColor};

pub enum GameState {
    Going,
    Win,
    Lose,
}

pub fn generate_answer(
    prev_answer: &mut String,
    random: &bool,
    day: &u32,
    answer_list: &Vec<String>,
) {
    if random {
        if day >= answer_list.len() + 1 {
            panic!("The degisnated day exceeds the number of candidate answers. Also you probably need a break...");
        }
        prev_answer = answer_list[day + 1];
    } else {
        let mut input: String;
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.len() != 5 {
                print!("Your input have to be 5 letters long.");
            } else if !input.chars().all(char::is_alphabetic) {
                print!("Your input contains non-letter characters.");
            } else {
                let capitalized_input = input.chars().to_uppercase().collect();
                if answer_list.contains(&capitalized_input) {
                    break;
                } else {
                    print!("Your input is not in the list of answers.");
                }
            }
        }
        prev_answer = &mut input;
    }
}

pub fn check_valid_entry(entry: &String, list: &Vec<String>) -> bool {
    if entry.len() != 5 {
        return false;
    }
    if !input.chars().all(char::is_ascii_uppercase(&self)) {
        return false;
    }
    if !list.contains(&input) {
        return false;
    }
    return true;
}

pub fn check_valid_attempt(
    attempt: &String,
    difficult: &bool,
    guess_list: &Vec<String>,
    prev_attempts: &AttemptState,
    alphabet: &AlphabetState
) -> bool {
    if difficult {
        if !guess_list.contains(attempt) {
            return false;
        }

        let char_attempt: [char; 5] = attempt.chars().collect();

        match prev_attempts.attempts.last() {
            None => {return true;}
            Some(last_attempt) => {
                for i in 0..5 {
                    if last_attempt[i].color == LetterColor::Green && last_attempt[i].letter != char_attempt[i] {
                        return false;
                    }
                }

                for i in 0..26 {
                    if alphabet.letters[i].color == LetterColor::Yellow {
                        for j in 0..5 {
                            if char_attempt[i] == alphabet.letters[i].letter {
                                break;
                            }
                            return false;
                        }
                    }
                }

                return true;
            }
        }
    } else {
        return guess_list.contains(attempt);
    }
}

pub fn check_end(att: &AttemptState, ans: &String) -> GameState {
    if block_array_to_string(att.attempts.last().unwrap()) == ans {
        return GameState::Win;
    } else {
        if att.attempts.len() < 6 {
            return GameState::Going;
        } else {
            return GameState::Lose;
        }
    }
}

fn block_array_to_string(block_array: &[Block; 5]) -> String {
    let mut new_string: String;
    for i in 0..5 {
        new_string.push(block_array[i].letter);
    }
    return new_string;
}
