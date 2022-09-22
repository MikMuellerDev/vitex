use std::{
    fs, io,
    path::Path,
    process::{Command, Stdio},
};

use log::info;

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

pub fn sync_templates(templates: &Vec<Template>, base_path: &Path) -> Result<(), io::Error> {
    let templates: Vec<&Template> = templates
        .iter()
        .filter(|template| !template.git_repository.is_empty())
        .collect();

    for template in templates {
        let repo_path = base_path
            .join("templates")
            .join(".cloned")
            .join(&template.id);
        if !repo_path.exists() {
            info!(
                "Cloning template `{}` from `{}`...",
                template.id, template.git_repository
            );
            let output = Command::new("git")
                .arg("clone")
                .arg(&template.git_repository)
                .arg(repo_path)
                .stderr(Stdio::inherit())
                .output()?;
            if !output.status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "could not clone git repo ({}) of template {}",
                        template.git_repository, template.id,
                    ),
                ));
            }
            info!("Successfully cloned template")
        } else {
            info!("Updating template `{}`...", template.id);
            let output = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("pull")
                .stderr(Stdio::inherit())
                .stdout(Stdio::piped())
                .output()?;
            if !output.status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "could not pull from git repo ({}) of template {}",
                        template.git_repository, template.id,
                    ),
                ));
            }
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim_end().trim_start() == "Already up to date." {
                info!("Template `{}` is up to date.", template.id)
            } else {
                println!("Template `{}` was updated:\n{stdout}", template.id)
            }
        }
    }
    Ok(())
}

pub fn purge_cloned(base_path: &Path) -> io::Result<()> {
    fs::remove_dir_all(base_path.join("templates").join(".cloned"))?;
    info!("Successfully deleted cloned templates");
    Ok(())
}
