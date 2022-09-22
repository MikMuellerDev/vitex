use std::{fmt::Display, fs, io, path::Path};

use crate::config::Template;

pub const REPLACE_KEYS: [&'static str; 3] = [
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
    MissingConfigTex {
        id: String,
        full_path: String,
    },
    MissingTemplate(String),
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
                    format!("Template `{id}` holds malformed config.tex (at `preamble/config.tex`):\n{details}"),
                Self::PathPrefixError { id, full_path } =>
                    format!("Invalid path-prefix for template `{id}`:\nPath prefix leads to nowhere (full path: `{full_path}`)"),
                    Self::MissingConfigTex { id, full_path } =>
                    format!("Template `{id}` is missing the file `preable/config.tex` (full path `{full_path}`)"),
                Self::IORead { id, path, io_error } => format!("Could not read file at `{path}` whilst validating template `{id}`:\n{io_error}"),
                Self::MissingTemplate(id) => format!("Template `{id}` is set-up but not yet installed:\nHINT: run `vitex templates sync` to address this issue")
            }
        )
    }
}

pub fn validate_templates(
    templates: &Vec<Template>,
    base_path: &Path,
) -> Result<(), ValidateError> {
    for template in templates {
        if template.git.repository.is_empty() {
            continue;
        }
        // Validate the current template
        template.validate(base_path)?;
    }
    Ok(())
}

impl Template {
    pub fn validate(&self, base_path: &Path) -> Result<(), ValidateError> {
        let template_path = base_path.join("templates").join(".cloned").join(&self.id);
        let path_with_prefix = template_path.join(&self.git.path_prefix);
        let config_tex_path = path_with_prefix.join("preamble").join("config.tex");
        // Test if the template exists
        if !template_path.exists() {
            return Err(ValidateError::MissingTemplate(self.id.clone()));
        };
        // Test if the path prefix is valid
        if !path_with_prefix.exists() {
            return Err(ValidateError::PathPrefixError {
                id: self.id.clone(),
                full_path: path_with_prefix
                    .to_str()
                    .expect("Path should be a valid String")
                    .to_string(),
            });
        }
        // Test if the template contains a `preable/config.tex`
        if !config_tex_path.exists() {
            return Err(ValidateError::MissingConfigTex {
                id: self.id.clone(),
                full_path: config_tex_path
                    .to_str()
                    .expect("Path should be a valid String")
                    .to_string(),
            });
        }
        // Test if the config.tex is well-formed
        let config_tex = match fs::read_to_string(&config_tex_path) {
            Ok(file) => file,
            Err(err) => {
                return Err(ValidateError::IORead {
                    id: self.id.clone(),
                    path: config_tex_path
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
            if !&config_tex.contains(replace_key) {
                return Err(ValidateError::ReplaceError {
                    id: self.id.clone(),
                    details: format!("Could not find / replace key `{replace_key}`"),
                });
            }
        }
        Ok(())
    }
}
