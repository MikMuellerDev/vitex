use std::{path::Path, process};

use clap::{command, Parser};
use cli::{Args, Command};
use log::{error, Level};
use loggerv::Logger;

use crate::cli::TemplateCommand;

mod cli;
mod config;
mod new;
mod templates;

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

    let base_path = config::file_path().unwrap_or_else(|| {
        error!("Could not determine a config-file path: do you have a home-directory?");
        process::exit(1);
    });
    let base_path = Path::new(&base_path);

    let conf = config::read_config(&base_path).unwrap_or_else(|err| {
        error!("Could not read or create config file: {err}");
        process::exit(1);
    });

    templates::create_templates_directory(&base_path).unwrap_or_else(|err| {
        error!(
            "Could not create templates directory (at `{}`): {err}",
            base_path
                .join("templates")
                .to_str()
                .expect("Path is expected to be a valid string")
        )
    });

    println!(
        "Templates: {}",
        &conf
            .templates
            .iter()
            .map(|template| template.id.clone())
            .collect::<Vec<String>>()
            .join("| ")
    );

    match args.command {
        Command::Templates(command) => match command {
            TemplateCommand::Sync => {
                templates::sync_templates(&conf.templates, base_path).unwrap_or_else(|err| {
                    error!("Could not sync templates: {err}");
                    process::exit(1);
                })
            },
            _ => todo!(),
        },
        Command::Config => println!(
            "Configuration file is located at: `{}`",
            base_path
                .join("config.toml")
                .to_str()
                .expect("Path is expected to be a valid string")
        ),
        Command::New { title, template } => new::new(&title, template).unwrap_or_else(|err| {
            eprintln!("Could not create new project: {err}");
            process::exit(1);
        }),
    }
}
