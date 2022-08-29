// TODO: debug

use std::io;

pub mod init;
pub mod internal_state;
pub mod json_state;
pub mod logic;
pub mod render;

use internal_state as is;
use json_state as js;

use is::AlphabetState;
use is::AttemptState;

fn main() {
    let mut loaded_config = init::load_config();
    let (guess_list, answer_list) = init::read_lists_and_shuffle();

    let mut answer: [char; 5];
    let mut attempt: String = String::new();

    loop {
        logic::generate_answer(
            &mut answer,
            &loaded_config.random,
            &loaded_config.day,
            &answer_list,
        );

        let mut round_attempt = AttemptState.new();
        let mut round_alphabet = AlphabetState.new();

        if () {
            js::update_from_json_stat();
        }

        loop {
            loop {
                io::stdin()
                    .read_line(&mut attempt)
                    .expect("Failed to read line");

                if logic::check_valid_attempt(&attempt, &loaded_config.difficult, &answer_list) {
                    break;
                } else {
                    print!("Invalid attempt. Please try again.")
                }
            }

            round_attempt.push_attempt();
            is::alphabet_state_update();

            if logic::check_end() {
                js::json_stat_append();
                break;
            } else {
                render::render_attempt();
            }
        }

        if render::render_end() {
            break;
        } else {
            is::update_new_round();
        }
    }
}
