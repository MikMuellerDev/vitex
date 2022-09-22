use std::{
    fmt::Display,
    fs, io,
    path::Path,
    process::{Command, Stdio},
};

use log::{debug, info};

use crate::{config::Template, templates::validate::validate_templates};

use super::validate::ValidateError;

pub enum SyncError {
    IO(io::Error),
    Git(String),
    Validate(ValidateError),
}

impl Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SyncError::IO(err) => format!("IO error: {err}"),
                Self::Validate(err) =>
                    format!("post-sync template validation detected an issue: {err}"),
                Self::Git(message) => format!("Git error: {message}"),
            }
        )
    }
}

impl From<io::Error> for SyncError {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<ValidateError> for SyncError {
    fn from(err: ValidateError) -> Self {
        Self::Validate(err)
    }
}

pub fn sync_git(templates: &Vec<Template>, cloned_path: &Path) -> Result<(), SyncError> {
    let git_templates: Vec<&Template> = templates
        .iter()
        .filter(|template| !template.git.repository.is_empty())
        .collect();
    // Iterate over the git templates
    for template in &git_templates {
        // The path were the repository is located
        let repo_path = cloned_path.join(&template.id);
        if !repo_path.exists() {
            debug!(
                "Template `{}` does not exist: cloning from `{}`...",
                template.id, template.git.repository
            );
            let output = Command::new("git")
                .arg("clone")
                .arg(&template.git.repository)
                .arg(repo_path)
                .stderr(Stdio::inherit())
                .output()?;
            if !output.status.success() {
                return Err(SyncError::Git(format!(
                    "could not clone git repo ({}) of template {}",
                    template.git.repository, template.id,
                )));
            }
            info!("Successfully cloned template")
        } else {
            debug!("Updating template `{}`...", template.id);
            let output = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("pull")
                .stderr(Stdio::inherit())
                .stdout(Stdio::piped())
                .output()?;
            if !output.status.success() {
                return Err(SyncError::Git(format!(
                    "could not pull from git repo ({}) of template {}",
                    template.git.repository, template.id,
                )));
            }
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim_end().trim_start() == "Already up to date." {
                info!("Template `{}` is up to date.", template.id)
            } else {
                info!("Template `{}` was updated:\n{stdout}", template.id)
            }
        }
    }
    debug!("Validating templates...");
    validate_templates(&templates, &cloned_path)?;
    info!(
        "Updated and scanned {} template(s). No issues detected.",
        &git_templates.len()
    );
    Ok(())
}

pub fn purge_cloned(cloned_path: &Path) -> io::Result<()> {
    fs::remove_dir_all(cloned_path)?;
    info!("Successfully deleted cloned templates");
    Ok(())
}
