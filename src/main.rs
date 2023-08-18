pub mod assets;
pub mod cli;

pub mod build;
mod utils;
pub mod yew;

use clap::Parser;
use cli::{Cli, TailWindYew};
use yew::yew_setup;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        TailWindYew::Add {
            path,
            input_name,
            output_name,
        } => yew_setup(&path, &input_name, &output_name),
    }
}
