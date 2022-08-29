/** This file deals with the initialization process upon launching the binary,
 * including generating the command line interface,
 * reading the configuration,
 * reading the two lists of words,
 * and shuffling the answer list if -r flag is passed in.
 * 
 * TODO: Write tests & debug borrowing interface and file reading
 */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate serde;

use clap::Parser;
use serde::de::DeserializeOwned;

use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(name = "Wordle_Rust")]
#[clap(author = "termina <termina.y.email@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "A game of Wordle, written in Rust!", long_about = None)]

pub struct ConfigFormat {
    #[clap(short, long, action)]
    pub random: bool,

    #[clap(short = 'D', long, action)]
    pub difficult: bool,

    #[clap(short = 't', long, action)]
    pub stats: bool,

    #[clap(short = 'o', long, action)]
    pub debug_output: bool,

    #[clap(short = 'd', long, value_parser)]
    pub day: Option<u32>,

    #[clap(short = 's', long, value_parser)]
    pub seed: Option<u64>,

    #[clap(short, long, value_parser, value_name = "FILE")]
    pub guess_list: Option<PathBuf>,

    #[clap(short, long, value_parser, value_name = "FILE")]
    pub answer_list: Option<PathBuf>,

    #[clap(short, long, value_parser, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[clap(short = 'S', long, value_parser, value_name = "FILE")]
    pub state: Option<PathBuf>,
}

struct JSONConfig {
    random: bool,
    difficult: bool,
    stats: bool,
    day: Option<u32>,
    seed: Option<u64>,
    guess_list: Option<String>,
    answer_list: Option<String>,
    state: Option<String>,
}

// Loads config, whether from the command line or from a JSON file.
// Passes ownership of the config object to the main function.
pub fn load_config() -> ConfigFormat {
    let mut loaded_config = ConfigFormat::parse();

    // Reads from JSON if -c is given.
    match loaded_config.config {
        Some(config_file_path) => {
            read_json_config::<JSONConfig, PathBuf>(config_file_path).unwrap();
        }
        None => {}
    }

    // Argument validity checking
    if !loaded_config.random && (loaded_config.day != None || loaded_config.seed != None) {
        panic!("Contradictory arguments: non-random game mode but a day or a seed is provided.");
    }

    loaded_config
}

// Read guess and answer lists, whether default or custom.
// Passes ownership of the two list BTreeSets to the main function.
pub fn read_lists_and_shuffle(
    guess_list_path: &Option<PathBuf>,
    answer_list_path: &Option<PathBuf>,
) -> (Vec<String>, Vec<String>) {
    let mut guess_list: Vec<String> = Vec::new();
    let mut answer_list: Vec<String> = Vec::new();

    match guess_list_path {
        None => {
            if let Ok(lines) = read_lines("./assets/default_guess_list") {
                for line in lines {
                    if let Ok(word) = line {
                        guess_list.push(word);
                    }
                }
            }
        }
        Some(custom_path) {

        }
    }


    (guess_list, answer_list)
}

fn read_json_config<T, P>(path: P) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let c: T = serde_json::from_reader(reader)?;

    // Return the ConfigFormat object
    Ok(c)
}

fn read_lines(filename: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

mod tests {
    // TODO
}