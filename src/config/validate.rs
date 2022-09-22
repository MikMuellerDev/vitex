use std::{fmt::Display, path::Path};

use super::Config;

pub enum ValidateError {
    DuplicateID(String),
    InvalidPath { id: String, path: String },
}

impl Display for ValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DuplicateID(id) => format!("ID `{id}` is duplicated but must be unique"),
                Self::InvalidPath { id, path } =>
                    format!("Template `{id}` was not found at local path: (expected: `{path}`):\nHINT: Template is expected to be local due to empty git repository"),
            }
        )
    }
}

impl Config {
    pub fn validate(self, custom_base_path: &Path) -> Result<Self, ValidateError> {
        let mut ids: Vec<&str> = Vec::with_capacity(self.templates.len());
        for template in &self.templates {
            if ids.contains(&template.id.as_str()) {
                return Err(ValidateError::DuplicateID(template.id.clone()));
            }
            ids.push(&template.id);
            if template.git.repository.is_empty() && !custom_base_path.join(&template.id).exists() {
                return Err(ValidateError::InvalidPath {
                    id: template.id.clone(),
                    path: custom_base_path
                        .join(&template.id)
                        .to_str()
                        .expect("Path should be valid String")
                        .to_string(),
                });
            }
        }
        Ok(self)
    }
}
