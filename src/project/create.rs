use std::{fmt::Display, fs, io, path::Path};

use log::warn;

use crate::{
    config::Template,
    templates::{TemplatePaths, ValidateError, REPLACE_KEYS},
};

pub enum Error {
    UnknownTemplate(String),
    NoTemplates,
    Validate(ValidateError),
    IORead { path: String, io_error: io::Error },
    IoWrite { path: String, io_error: io::Error },
    DirExists(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IORead { path, io_error } =>
                    format!("Could not read file (at `{path}`): {io_error}"),
                Self::IoWrite { path, io_error } =>
                    format!("Could not write to file (at `{path}`): {io_error}"),
                Self::Validate(err) => format!("Cannot use invalid project: {err}"),
                Self::UnknownTemplate(id) => format!("Template `{id}` is invalid"),
                Self::NoTemplates => "There are currently 0 templates.\nAt least 1 template is required to use this tool".to_string(),
                Self::DirExists(path) => format!("Will not create project directory at `{path}`: directory already exists")
            }
        )
    }
}

impl From<ValidateError> for Error {
    fn from(err: ValidateError) -> Self {
        Self::Validate(err)
    }
}

/// Creates a new project
pub fn create(
    templates: &Vec<Template>,
    template_id: Option<&str>,
    title: &str,
    author: &str,
    subtitle: Option<&str>,
    template_paths: &TemplatePaths,
    destination: &Path,
) -> Result<(), Error> {
    // Check if there are templates
    if templates.len() == 0 {
        return Err(Error::NoTemplates);
    }
    // Find the correct template
    let template_id = template_id.unwrap_or_else(|| &templates[0].id);
    let template = match templates.iter().find(|template| template.id == template_id) {
        Some(found) => found,
        None => return Err(Error::UnknownTemplate(template_id.to_string())),
    };
    // Check if the path already exists
    let destination = destination.join(title.replace(' ', "_").replace("/", "\\"));
    if destination.exists() {
        return Err(Error::DirExists(
            destination
                .to_str()
                .expect("Path should be a String")
                .to_string(),
        ));
    }
    // Validate the template in order to sort out some errors
    let template_path = match template.git.repository.is_empty() {
        true => template_paths.custom.join(&template.id),
        false => template_paths
            .cloned
            .join(&template.id)
            .join(&template.git.path_prefix),
    };
    // Validate the template
    template.validate(&template_path)?;
    // Copy the entire project to the destination
    if let Err(err) = copy_dir_all(&template_path, &destination) {
        return Err(Error::IoWrite {
            path: destination
                .to_str()
                .expect("Path should be a String")
                .to_string(),
            io_error: err,
        });
    }
    // Replace all the placeholders
    let config_tex_path = destination.join("preamble").join("config.tex");
    let main_tex_path = destination.join("main.tex");
    if config_tex_path.exists() {
        replace_placeholders_in_file(
            &config_tex_path,
            title,
            subtitle.unwrap_or_else(|| title),
            author,
        )?;
    } else if main_tex_path.exists() {
        replace_placeholders_in_file(
            &main_tex_path,
            title,
            subtitle.unwrap_or_else(|| title),
            author,
        )?;
    } else {
        warn!("Project contains no `main.tex` or `preamble/config.tex`")
    }
    Ok(())
}

/// Replaces the title, subtitle and author placeholders in a file
fn replace_placeholders_in_file(
    file_path: &Path,
    title: &str,
    subtitle: &str,
    author: &str,
) -> Result<(), Error> {
    // Read the raw file contents
    let raw_config_tex = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(Error::IORead {
                path: file_path
                    .to_str()
                    .expect("Path should be a valid String")
                    .to_string(),
                io_error: err,
            })
        }
    };
    // Write the contents to the file whilst replacing them
    if let Err(err) = fs::write(
        file_path,
        // Replace all keys in the file
        raw_config_tex
            .replace(REPLACE_KEYS[1], subtitle)
            .replace(REPLACE_KEYS[0], title)
            .replace(REPLACE_KEYS[2], author),
    ) {
        return Err(Error::IoWrite {
            path: file_path
                .to_str()
                .expect("Path should be a String")
                .to_string(),
            io_error: err,
        });
    };
    Ok(())
}

/// Like `cp -r` on Unix platforms:
/// Recursively copies a directory and all its contents
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
