use std::{fmt::Display, path::Path};

use super::Config;

pub enum ValidateError {
    DuplicateID(String),
    InvalidPath { id: String, path: String },
    GitAndPathDefined(String),
    NoSources(String),
}

impl Display for ValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoSources(id) => format!("template with ID `{id}` has no sources"),
                Self::DuplicateID(id) => format!("ID `{id}` is duplicated but must be unique"),
                Self::InvalidPath { id, path } =>
                    format!("invalid local path (`{path}`) at ID `{id}`"),
                Self::GitAndPathDefined(id) => format!(
                    "both git-repository and local path specified for ID `{id}`: this is ambigious"
                ),
            }
        )
    }
}

impl Config {
    pub fn validate(self) -> Result<Self, ValidateError> {
        let mut ids: Vec<&str> = Vec::with_capacity(self.templates.len());
        for template in &self.templates {
            if ids.contains(&template.id.as_str()) {
                return Err(ValidateError::DuplicateID(template.id.clone()));
            }
            ids.push(&template.id);
            if !template.local_path.is_empty() && !template.git.repository.is_empty() {
                return Err(ValidateError::GitAndPathDefined(template.id.clone()));
            }
            if template.local_path.is_empty() && template.git.repository.is_empty() {
                return Err(ValidateError::NoSources(template.id.clone()));
            }
            if !template.local_path.is_empty() && !Path::new(&template.local_path).exists() {
                return Err(ValidateError::InvalidPath {
                    id: template.id.clone(),
                    path: template.local_path.clone(),
                });
            }
        }
        Ok(self)
    }
}
