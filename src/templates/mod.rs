mod sync;
mod validate;

pub use sync::{purge_cloned, sync_git};
pub use validate::{validate_templates, ValidateError, REPLACE_KEYS};

use std::{
    env, fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use crate::config::Template;

pub struct TemplatePaths {
    pub custom: PathBuf,
    pub cloned: PathBuf,
}

pub fn create_templates_directory(custom: &PathBuf, cloned: &PathBuf) -> io::Result<()> {
    // Custom tempplates
    if !custom.exists() {
        fs::create_dir_all(custom)?
    }
    if !cloned.exists() {
        fs::create_dir_all(cloned)?
    }
    Ok(())
}

pub fn template_paths(base_path: &Path) -> io::Result<TemplatePaths> {
    // Custom templates
    let custom_templates_path = base_path.join("custom_templates");
    // Cloned templates
    let cloned_templates_path = match env::var("HOME") {
        Ok(home) => Path::new(home.as_str())
            .join(".local")
            .join("share")
            .join("vitex")
            .join("clone"),
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "$HOME environent variable undefied: do you have a home?",
            ))
        }
    };
    Ok(TemplatePaths {
        custom: custom_templates_path,
        cloned: cloned_templates_path,
    })
}

pub fn list_templates(templates: &[Template]) {
    println!(
        "=== Templates ===\n{}",
        templates
            .iter()
            .map(|template| format!(" - {}", template.id))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
