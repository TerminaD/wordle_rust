/** This file prints stuff to the command line
 *
 * TODO: debug, test
 */

use colored::Colorize;

use std::io;

use crate::{
    internal_state::{AttemptState, LetterColor, AlphabetState},
    logic::GameState, json_state::calculate_stats,
};

pub fn render_attempt(data: &AttemptState, alpha: &AlphabetState) {
    clearscreen::clear().expect("Failed to clear terminal.");

    let attempt_size: usize = data.len();

    for i in 0..attempt_size {
        for j in 0..5 {
            let curr_block = ((data.attempts)[i])[j];
            match curr_block.color {
                LetterColor::Green => println!("{} ", curr_block.letter.to_string().white().on_green()),
                LetterColor::Yellow => println!("{} ", curr_block.letter.to_string().white().on_yellow()),
                LetterColor::Red => println!("{} ", curr_block.letter.to_string().white().on_red()),
                LetterColor::Grey => println!("{} ",curr_block.letter.to_string().white().on_truecolor(50, 50, 50)),
            }
        }
        print!("{}", " ");
    }

    for i in attempt_size..6 {
        print!("{}", "_ _ _ _ _");
    }

    for curr_block in alpha.letters {
        match curr_block.color {
            LetterColor::Green => println!("{} ", curr_block.letter.to_string().green()),
            LetterColor::Yellow => println!("{} ", curr_block.letter.to_string().yellow()),
            LetterColor::Red => println!("{} ", curr_block.letter.to_string().red()),
            LetterColor::Grey => println!("{} ",curr_block.letter.to_string().truecolor(50, 50, 50)),
        }
    }
        
    }
}
pub fn render_end(state: &GameState, data: &AttemptState, answer: &String, difficult: &bool) -> bool {
    clearscreen::clear().expect("Failed to clear terminal.");

    match state {
        GameState::Win => print!("{}", "You win!"),
        GameState::Lose => print!("{}", "You lose!"),
        _ => panic!("Unexpected behavior: game both ongoing and finished."),
    }

    print!("The answer is: {}", answer);

    let attempt_size: usize = data.len();

    for i in 0..attempt_size {
        for j in 0..5 {
            let curr_block = ((data.attempts)[i])[j];
            match curr_block.color {
                LetterColor::Green => println!("{} ", curr_block.letter.to_string().white().on_green()),
                LetterColor::Yellow => println!("{} ", curr_block.letter.to_string().white().on_yellow()),
                LetterColor::Red => println!("{} ", curr_block.letter.to_string().white().on_red()),
                LetterColor::Grey => println!("{} ",curr_block.letter.to_string().white().on_truecolor(50, 50, 50)),
            }
        }
        print!("{}", " ");
    }

    for i in attempt_size..6 {
        print!("{}", "_ _ _ _ _");
    }

    if difficult {
        render_stats(calculate_stats());
    }

    print!("Press \"R\" to play another round, or press any other key to exit.");

    let mut responce: String;

    io::stdin()
        .read_line(&mut responce)
        .expect("Failed to read responce");

	match responce {
		"r" => return true,
		responce.len() > 1 => panic("You've put in more than one letter!"),
		_ => return false,
	}
}

fn render_stats() {

}

mod tests {}
