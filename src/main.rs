use std::{path::Path, process};

use clap::Parser;
use cli::{Args, Command, ProjectCommand};
use log::{error, info, Level};
use loggerv::Logger;

use crate::cli::TemplateCommand;
use log::debug;

mod cli;
mod config;
mod project;
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
        error!("Could not determine a config base path: do you have a home-directory?");
        process::exit(1);
    });
    let base_path = Path::new(&base_path);

    // Create the template directories
    let template_paths = templates::template_paths(base_path).unwrap_or_else(|err| {
        error!("Could not determine template paths: {err}");
        process::exit(1);
    });
    templates::create_templates_directory(&template_paths.custom, &template_paths.cloned)
        .unwrap_or_else(|err| {
            error!(
                "Could not create template directories (at `{}` and `{}`): {err}",
                template_paths
                    .custom
                    .to_str()
                    .expect("Path should be a String"),
                template_paths
                    .cloned
                    .to_str()
                    .expect("Path should be a String")
            )
        });
    // Read or create the config file
    let conf = config::read_config(&base_path.join("config.toml"), &template_paths.custom)
        .unwrap_or_else(|err| {
            error!("Could not read or create config file: {err}");
            process::exit(1);
        });

    match args.command {
        Command::Templates(command) => match command {
            TemplateCommand::Sync => {
                debug!("Syncing {} templates...", conf.templates.len());
                templates::sync_git(&conf.templates, &template_paths.cloned).unwrap_or_else(|err| {
                    error!("Could not sync templates: {err}");
                    process::exit(1);
                })
            }
            TemplateCommand::Validate => {
                for path in [template_paths.cloned, template_paths.custom] {
                    templates::validate_templates(&conf.templates, &path).unwrap_or_else(|err| {
                        error!("Validation detected an issue:\n{err}");
                        process::exit(1);
                    });
                }
                info!(
                    "Scanned {} template(s). No issues detected.",
                    conf.templates.len()
                );
            }
            TemplateCommand::List => templates::list_templates(&conf.templates),
            TemplateCommand::Purge => templates::purge_cloned(&template_paths.cloned)
                .unwrap_or_else(|err| {
                    error!("Could not purge cloned templates: {err}");
                    process::exit(1);
                }),
        },
        Command::Project(command) => match command {
            ProjectCommand::New {
                title,
                subtitle,
                template,
                author,
            } => project::create(
                &conf.templates,
                template.as_deref(),
                &title,
                &author.unwrap_or(conf.author_name),
                subtitle.as_deref(),
                &template_paths,
                Path::new(""),
            )
            .unwrap_or_else(|err| {
                eprintln!("Could not create new project: {err}");
                process::exit(1);
            }),
        },
        Command::Config => println!(
            "Configuration file is located at: `{}`",
            base_path
                .join("config.toml")
                .to_str()
                .expect("Path is expected to be a valid string")
        ),
    }
}
