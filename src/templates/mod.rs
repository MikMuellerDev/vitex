mod sync;
mod validate;

pub use sync::{purge_cloned, sync_git};
pub use validate::{validate_templates, ValidateError, REPLACE_KEYS};

use std::{fs, io, path::Path};

use crate::config::Template;

pub fn create_templates_directory(base_path: &Path) -> Result<(), io::Error> {
    let local_templates_path = base_path.join("templates").join("local");
    if !local_templates_path.exists() {
        fs::create_dir_all(local_templates_path)?
    }
    let cloned_templates_path = base_path.join("templates").join(".cloned");
    if !cloned_templates_path.exists() {
        fs::create_dir_all(cloned_templates_path)?
    }
    Ok(())
}

pub fn list(templates: &Vec<Template>) {
    println!(
        "=== Templates ===\n{}",
        templates
            .iter()
            .map(|template| format!(" - {}", template.id))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
