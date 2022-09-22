use std::process;

use clap::Parser;
use cli::{Args, Command};
use log::{Level, error};
use loggerv::Logger;

mod cli;
mod config;
mod new;

fn main() {
    let args = Args::parse();
    Logger::new()
        .max_level(if args.verbose {
            Level::Trace
        } else {
            Level::Info
        })
        .colors(true)
        .level(true)
        .module_path_filters(vec![env!("CARGO_PKG_NAME").replace('-', "_")])
        .module_path(false)
        .init()
        .unwrap();

    let config_path = config::file_path().unwrap_or_else(|| {
        error!("");
        process::exit(1);
    });

    match args.command {
        Command::New { title, template } => new::new(&title, template),
    }
    .unwrap_or_else(|err| {
        eprintln!("An error occured: {err}");
        process::exit(1);
    })
}
