use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: TailWindYew,
}

#[derive(Subcommand)]
pub enum TailWindYew {
    /// Adds tailwind to an existing yew project
    Add {
        /// Optional - Path for the tailwind input and output css files
        #[clap(short, long, default_value = "styles")]
        path: String,

        /// Optional - Input css name
        /// You must add the extension ie. [my_input.css]
        #[clap(short, long, default_value = "input.css")]
        input_name: String,

        /// Optional - Output css name
        /// You must add the extension ie. [my_output.css]
        #[clap(short, long, default_value = "output.css")]
        output_name: String,
    },
}
