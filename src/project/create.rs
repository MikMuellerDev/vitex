use std::{fmt::Display, fs, io, path::Path};

use crate::{
    config::Template,
    templates::{ValidateError, REPLACE_KEYS},
};

pub enum Error {
    UnknownTemplate(String),
    NoTemplates,
    Validate(ValidateError),
    IORead { path: String, io_error: io::Error },
    IoWrite { path: String, io_error: io::Error },
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
            }
        )
    }
}

impl From<ValidateError> for Error {
    fn from(err: ValidateError) -> Self {
        Self::Validate(err)
    }
}

fn create_helper(
    template: &Template,
    author: &str,
    title: &str,
    subtitle: &str,
    base_path: &Path,
    destination: &Path,
) -> Result<(), Error> {
    let template_path = match template.git.repository.is_empty() {
        true => base_path.join("templates").join("local").join(&template.id),
        false => base_path
            .join("templates")
            .join(".cloned")
            .join(&template.id)
            .join(&template.git.path_prefix),
    };
    let config_tex_path = template_path.join("preamble").join("config.tex");

    // Read the raw config.tex file
    let raw_config_tex = match fs::read_to_string(&config_tex_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(Error::IORead {
                path: config_tex_path
                    .to_str()
                    .expect("Path should be a valid String")
                    .to_string(),
                io_error: err,
            })
        }
    };
    // Copy the entire project to the destination
    if let Err(err) = copy_dir_all(template_path, destination.join(title)) {
        return Err(Error::IoWrite {
            path: destination
                .to_str()
                .expect("Path should be a String")
                .to_string(),
            io_error: err,
        });
    }
    println!("aa");
    // Write the replaced config.tex to the destination
    if let Err(err) = fs::write(
        destination.join(title).join("preamble"),
        // Replace all keys in the config file
        raw_config_tex
            .replace(REPLACE_KEYS[0], title)
            .replace(REPLACE_KEYS[1], subtitle)
            .replace(REPLACE_KEYS[2], author),
    ) {
        return Err(Error::IoWrite {
            path: destination
                .to_str()
                .expect("Path should be a String")
                .to_string(),
            io_error: err,
        });
    };
    Ok(())
}

pub fn create(
    templates: &Vec<Template>,
    template_id: Option<&str>,
    title: &str,
    author: &str,
    subtitle: Option<&str>,
    base_path: &Path,
    destination: &Path,
) -> Result<(), Error> {
    if templates.len() == 0 {
        return Err(Error::NoTemplates);
    }
    let template_id = template_id.unwrap_or_else(|| &templates[0].id);

    let template = match templates.iter().find(|template| template.id == template_id) {
        Some(found) => found,
        None => return Err(Error::UnknownTemplate(template_id.to_string())),
    };
    // Validate the template to sort out some errors
    template.validate(base_path)?;
    create_helper(
        template,
        author,
        title,
        subtitle.unwrap_or_else(|| title),
        base_path,
        destination,
    )?;
    Ok(())
}

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
