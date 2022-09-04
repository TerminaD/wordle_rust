pub mod init;
pub mod internal_state;
pub mod json_state;
pub mod logic;
pub mod render;

use std::io;

use internal_state as is;
use json_state as js;

use is::AlphabetState;
use is::AttemptState;
use logic::GameState;

fn main() {
    let mut loaded_config = init::load_config();
    let (guess_list, answer_list) = init::read_lists_and_shuffle(
        loaded_config.guess_list,
        loaded_config.answer_list,
        &loaded_config.random,
        &loaded_config.seed,
    );

    let mut answer: [char; 5];
    let mut attempt: String = String::new();

    loop {
        logic::generate_answer(
            &mut answer,
            &loaded_config.random,
            &loaded_config.day,
            &answer_list,
        );

        let mut round_attempt = AttemptState::new();
        let mut round_alphabet = AlphabetState::new();

        match loaded_config.state {
            None => {}
            Some(stat_path) => {
                (round_attempt, answer) =
                    js::update_from_json_stat(stat_path, &answer_list, &guess_list);
            }
        }

        let curr_state: GameState;

        loop {
            loop {
                io::stdin()
                    .read_line(&mut attempt)
                    .expect("Failed to read line");

                if logic::check_valid_attempt(
                    &attempt,
                    &loaded_config.difficult,
                    &answer_list,
                    &round_attempt,
                    &round_alphabet,
                ) {
                    break;
                } else {
                    print!("Invalid attempt. Please try again.")
                }
            }

            round_alphabet.update(round_attempt.push_attempt());

            curr_state = logic::check_end(&round_attempt, &answer);

            match curr_state {
                Going => {
                    render::render_attempt(&round_attempt, &round_alphabet);
                }
                _ => {
                    js::json_stat_append(loaded_config.state, &round_attempt, &answer);
                    break;
                }
            }
        }

        if render::render_end(&curr_state, &round_attempt, &answer, &loaded_config.state) {
            break;
        } else {
            is::update_new_round(&mut loaded_config);
        }
    }
}
