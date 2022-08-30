/** This file deals with logic involving
 * answer generating, checking for an attempt's validity, and determining when the game ends.
 * 
 * TODO: check_end, tests, debug, check_valid when difficult
 */

use std::io;

pub enum GameState {
    Going, 
    Win, 
    Lose
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
        prev_answer = input;
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
    guess_list: &Vec<String>
) -> bool {
    if difficult {
        
        
    } else {
        return guess_list.contains(attempt);
    }
}

pub fn check_end() -> GameState {

}

mod tests {}
