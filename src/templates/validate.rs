use std::{
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};

use log::warn;

use crate::config::Template;

pub const REPLACE_KEYS: [&str; 3] = [
    "VITEX_TITLE_PLACEHOLDER",
    "VITEX_SUBTITLE_PLACEHOLDER",
    "VITEX_AUTHOR_PLACEHOLDER",
];

pub enum ValidateError {
    ReplaceError {
        id: String,
        details: String,
    },
    PathPrefixError {
        id: String,
        full_path: String,
    },
    MissingConfigAndMainTex {
        id: String,
        full_path: String,
    },
    NotFound(String),
    NotCloned(String),
    IORead {
        id: String,
        path: String,
        io_error: io::Error,
    },
}

impl Display for ValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ReplaceError { id, details } =>
                    format!("Template `{id}` holds malformed config.tex or main.tex (at `preamble/config.tex` or `main.tex`):\n{details}"),
                Self::PathPrefixError { id, full_path } =>
                    format!("Invalid path-prefix for template `{id}`:\nPath prefix leads to nowhere (full path: `{full_path}`)"),
                    Self::MissingConfigAndMainTex { id, full_path } =>
                    format!("Template `{id}` is missing the file `preable/config.tex` or `main.tex` (full path `{full_path}`)"),
                Self::IORead { id, path, io_error } => format!("Could not read file at `{path}` whilst validating template `{id}`:\n{io_error}"),
                Self::NotFound(id) => format!("Template `{id}` is set-up but not found locally:\nHINT: check if the template is present at the correct path"),
                Self::NotCloned(id) => format!("Template `{id}` is set-up but not yet installed:\nHINT: run `vitex templates sync` to address this issue"),
            }
        )
    }
}

pub fn validate_templates(
    templates: &Vec<Template>,
    templates_path: &Path,
) -> Result<(), ValidateError> {
    for template in templates {
        if template.git.repository.is_empty() {
            continue;
        }
        // Create a repository path if the template uses git
        let repo_path = if template.git.repository.is_empty() {
            None
        } else {
            Some(templates_path.join(&template.id))
        };
        // Validate the current template
        template.validate(
            &templates_path
                .join(&template.id)
                .join(&template.git.path_prefix),
            repo_path.as_ref(),
        )?;
    }
    Ok(())
}

impl Template {
    pub fn validate(
        &self,
        template_path: &Path,
        repository_path: Option<&PathBuf>,
    ) -> Result<(), ValidateError> {
        if let Some(repository_path) = repository_path {
            // Test if the template is cloned
            if !repository_path.join(&self.id).exists() {
                return Err(ValidateError::NotCloned(self.id.clone()));
            };
            // Test if the path prefix is valid
            if !template_path.exists() {
                return Err(ValidateError::PathPrefixError {
                    id: self.id.clone(),
                    full_path: template_path
                        .to_str()
                        .expect("Path should be a valid String")
                        .to_string(),
                });
            }
        } else {
            // Test if the template can be found locally
            if !template_path.exists() {
                return Err(ValidateError::NotFound(self.id.clone()));
            };
        }
        // Test if the template contains a `preable/config.tex` or `main.tex`
        let config_tex_path = template_path.join("preamble").join("config.tex");
        let main_tex_path = template_path.join("main.tex");

        if !config_tex_path.exists() && !main_tex_path.exists() {
            return Err(ValidateError::MissingConfigAndMainTex {
                id: self.id.clone(),
                full_path: main_tex_path
                    .to_str()
                    .expect("Path should be a valid String")
                    .to_string(),
            });
        }
        // Test if the config.tex or the main.tex is well-formed
        if config_tex_path.exists() {
            validate_tex_file(&self.id, &config_tex_path)?;
        } else if main_tex_path.exists() {
            validate_tex_file(&self.id, &main_tex_path)?;
        } else {
            warn!("Project contains no `main.tex` or `preamble/config.tex`")
        }
        Ok(())
    }
}

fn validate_tex_file(id: &str, path: &Path) -> Result<(), ValidateError> {
    let file_contents = match fs::read_to_string(&path) {
        Ok(file) => file,
        Err(err) => {
            return Err(ValidateError::IORead {
                id: id.to_string(),
                path: path
                    .to_str()
                    .expect("Path should be a valid String")
                    .to_string(),
                io_error: err,
            })
        }
    };
    // Check if the file contains various keys
    for replace_key in REPLACE_KEYS {
        // Test if the current replace key can be found
        if !file_contents.contains(replace_key) {
            return Err(ValidateError::ReplaceError {
                id: id.to_string(),
                details: format!("Could not find / replace key `{replace_key}`"),
            });
        }
    }
    Ok(())
}
