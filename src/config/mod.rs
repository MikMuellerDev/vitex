mod errors;
mod read;
mod validate;

pub use read::{file_path, read_config};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub author_name: String,
    pub templates: Vec<Template>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            author_name: "John Doe".to_string(),
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
                repository: "https://github.com/MikMuellerDev/vitex".to_string(),
                path_prefix: "templates/normal".to_string(),
            },
        }
    }
}
