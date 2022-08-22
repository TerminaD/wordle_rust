/** This file generates the command line interface
  * and reads the configuration from the command line.
  */

use clap::Parser;

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

    #[clap(short, long, value_parser)]
    pub day: Option<u16>,

    #[clap(short, long, value_parser)]
    pub seed: Option<u64>,

    #[clap(short, long, value_parser, value_name = "FILE", default_value_t = String::from("./src/assets/answers_list.txt"))]
    pub answer_list: String,

    #[clap(short, long, value_parser, value_name = "FILE", default_value_t = String::from("./src/assets/guesses_list.txt"))]
    pub guess_list: String,

    #[clap(short, long, value_parser, value_name = "FILE")]
    pub state: Option<String>,
}

pub fn parse_cli_input() -> ConfigFormat {
    let mut loaded_config = ConfigFormat::parse();
    loaded_config
}
