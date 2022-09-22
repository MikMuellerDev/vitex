mod errors;
mod read;
mod validate;

pub use read::{file_path, read_config};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub templates: Vec<Template>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            templates: vec![Template::default()],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub local_path: String,
    pub git: TemplateGitConfig,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateGitConfig {
    pub repository: String,
    pub path_prefix: String,
}

impl Default for Template {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            local_path: "".to_string(),
            git: TemplateGitConfig {
                repository: "git@github.com:foo/bar".to_string(),
                path_prefix: "src/normal-template".to_string(),
            },
        }
    }
}
