/** This file defines the structure of all guesses and the alphabet during each round
 * and also updates them when needed.
 *
 * TODO: tests, debug
 */

use std::collections::HashMap;

use crate::init::ConfigFormat;

#[derive(PartialEq, PartialOrd)]
pub enum LetterColor {
    Green,
    Yellow,
    Red,
    Grey,
}

pub struct Block {
    pub letter: char,
    pub color: LetterColor,
}

impl Block {
    pub fn new(letter: char, new_color: LetterColor) -> Block {
        if !letter.is_alphabetic() {
            panic!("The character you've entered is not a letter!");
        }

        let capitalized_letter = letter.to_ascii_uppercase();

        Block {
            letter: capitalized_letter,
            color: new_color,
        }
    }
}

pub struct AttemptState {
    pub attempts: Vec<[Block; 5]>,
}

impl AttemptState {
    pub fn new() {
        AttemptState {
            attempts: ()
        }
    }

    pub fn len(&self) -> usize {
        self.attempts.len()
    }

    pub fn push_attempt(&mut self, attempt: &String, answer: &[char; 5]) -> [Block; 5] {
        let mut attempt_iter = attempt.chars();
        let mut new_attempt: [Block; 5];
        let mut answer_hashmap: HashMap<char, u8>;
        let mut curr_color: LetterColor;

        for i in 0..5 {
            answer_hashmap[&answer[i]] += 1;
        }

        // Green & Grey
        for i in 0..5 {
            let letter = attempt_iter.next().unwrap();

            if letter == answer[i] {
                new_attempt[i] = Block::new(letter, LetterColor::Green);
                answer_hashmap[&letter] -= 1;
            } else {
                new_attempt[i] = Block::new(letter, LetterColor::Grey);
            }
        }

        // Yellow & Red
        for i in 0..5 {
            if is_in_answer(&new_attempt[i].letter, &answer) {
                if answer_hashmap[&new_attempt[i].letter] > 0 {
                    new_attempt[i].color = LetterColor::Yellow;
                    answer_hashmap[&new_attempt[i].letter] -= 1;
                } else {
                    new_attempt[i].color = LetterColor::Red;
                }
            }
        }

        self.attempts.push(new_attempt);
        return new_attempt;
    }
}

fn is_in_answer(curr: &char, answer: &[char; 5]) -> bool {
    for i in 0..5 {
        if curr == answer[i] {
            return true;
        }
    }
    return false;
}

pub struct AlphabetState {
    pub letters: [Block; 26],
}

impl AlphabetState {
    pub fn update(&mut self, new_attempt: &[Block; 5]) {
        let mut prev_color: &mut LetterColor;
        let mut curr_color: &LetterColor;
        for i in 0..5 {
            let curr_index = (new_attempt[i].letter.to_digit(36).unwrap() - 10) as usize;
            prev_color = &mut self.letters[curr_index].color;
            curr_color = &new_attempt[i].color;

            if prev_color > &mut curr_color {
                prev_color = &mut curr_color;
            }
        }
    }
}

pub fn update_new_round(curr_config: &mut ConfigFormat) {
    curr_config.day += 1;
}