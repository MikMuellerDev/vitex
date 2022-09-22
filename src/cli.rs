use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The path where the configuration file should be located
    #[clap(short, long, value_parser)]
    pub config_path: Option<String>,
    /// If set, more information will be printed to the console
    #[clap(short, long, value_parser, global = true)]
    pub verbose: bool,
    /// The subcommand to execute
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Outputs the location of the program's configuration file
    Config,
    New {
        /// The project's title
        title: String,
        /// The project template
        #[clap(short, long, value_parser)]
        template: Option<String>,
    },
}
