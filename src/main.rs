pub mod init;
pub mod logic;
pub mod render;
pub mod state;
pub mod tests;

fn main() {
    let mut loaded_config = init::load_config();
    let (guess_list, answer_list) = init::read_lists_and_shuffle();

    let mut answer: String = String::new();
    let mut attempt: String = String::new();
    loop {
        logic::generate_answer(
            &mut answer,
            &loaded_config.random,
            &loaded_config.day,
            &answer_list,
        );

        loop {
            loop {
                // TODO: read attempt from stdin

                if logic::check_valid(&attempt, &loaded_config.difficult, &answer_list) {
                    break;
                } else {
                    print!("Invalid attempt. Please try again.")
                }
            }

            state::update();

            if logic::check_end() {
                break;
            } else {
                render::render_attempt();
            }
        }

        if render::render_end() {
            break;
        } else {
            state::update_new_round();
        }
    }
}
