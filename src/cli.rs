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
    /// Templates subcommands
    #[clap(subcommand)]
    Templates(TemplateCommand),
    /// Project subcommands
    #[clap(subcommand)]
    Project(ProjectCommand),
}

#[derive(Subcommand)]
pub enum TemplateCommand {
    /// Downloads and syncs the specified templates
    Sync,
    /// Validates the local templates
    Validate,
    /// Lists the set-up templates
    List,
    /// Purges all cloned templates (does not affect local templates)
    Purge,
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    /// Creates a new LaTex project
    New {
        /// The project's title
        title: String,
        /// The project's subtitle
        #[clap(short, long, value_parser)]
        subtitle: Option<String>,
        /// The project's template
        #[clap(short, long, value_parser)]
        template: Option<String>,
        /// The project's author(s)
        #[clap(short, long, value_parser)]
        author: Option<String>,
    },
}
