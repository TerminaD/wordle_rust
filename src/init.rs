/** This file deals with the initialization process upon launching the binary,
 * including generating the command line interface, 
 * reading the configuration, 
 * reading the two lists of words,
 * and shuffling the answer list if -r flag is passed in.
 */
use clap::Parser;
use std::collections::BTreeSet;

#[derive(Parser)]
#[clap(name = "Wordle_Rust")]
#[clap(author = "termina <termina.y.email@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "A game of Wordle, written in Rust!", long_about = None)]

pub struct ConfigFormat {
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub config: Option<String>,

    #[clap(short, long, action)]
    pub random: bool,

    #[clap(short, long, action)]
    pub difficult: bool,

    #[clap(short, long, action)]
    pub stats: bool,

    #[clap(short, long, value_parser, default_value_t = 1)]
    pub day: u32,

    #[clap(short, long, value_parser)]
    pub seed: Option<u64>,

    #[clap(short, long, value_parser, value_name = "FILE", default_value_t = String::from("./assets/default_answer_list.txt"))]
    pub answer_list: String,

    #[clap(short, long, value_parser, value_name = "FILE", default_value_t = String::from("./assets/default_guess_list.txt"))]
    pub guess_list: String,

    #[clap(short, long, value_parser, value_name = "FILE")]
    pub state: Option<String>,
}

// Loads config, whether from the command line or from a JSON file.
// Passes ownership of the config object to the main function.
pub fn load_config() -> ConfigFormat {
    let mut loaded_config = ConfigFormat::parse();
    
    match loaded_config.config {
        Some(config_file_patch) => {
            read_json_config(&mut loaded_config);
        }
        None => {}
    }

    loaded_config
}

// Read guess and answer lists, whether default or custom.
// Passes ownership of the two list BTreeSets to the main function.
pub fn read_lists_and_shuffle() -> (BTreeSet<String>, BTreeSet<String>) {

}

fn read_json_config(prev_config: &mut ConfigFormat) {

}
