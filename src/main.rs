enum LetterState {
    Unknown, 
    Excessive, // Note that a letter that isn't present in the answer also counts as excessive
    Misplaced, 
    Correct
}

struct Config {

}

fn main() {
    
}

fn set_answer(is_random: &bool, day: &Option<u16>, seed: &Option<u64>) -> String {
    // TODO:
    // Randomly choose a word from vec of possible answers according to day and seed
    // or load appointed answer
}

fn check_case(input: &String, answer: &String) -> [LetterState] {

}

fn print_welcome() {

}

fn print_case() {

}

fn print_endgame(print_statistic: &bool) {

}

fn parse_cli_input() {
    // Read config from command line
    // or from JSON file if -c flag is specified
}

fn valid_guess(input: &String, difficult: &bool) -> bool {

}

fn initialize_word_lists(guess_list: &Option<String>, answer_list: &Option<String>) {

}

fn write_round_to_json() {

}

