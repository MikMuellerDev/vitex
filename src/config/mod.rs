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
        let repository = env!("CARGO_PKG_REPOSITORY");

        Self {
            author_name: "John Doe".to_string(),
            templates: vec![
                Template {
                    id: "normal".to_string(),
                    git: {
                        TemplateGitConfig {
                            repository: repository.to_string(),
                            path_prefix: "templates/normal".to_string(),
                        }
                    },
                },
                Template {
                    id: "blank".to_string(),
                    git: {
                        TemplateGitConfig {
                            repository: repository.to_string(),
                            path_prefix: "templates/blank".to_string(),
                        }
                    },
                },
            ],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub git: TemplateGitConfig,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateGitConfig {
    pub repository: String,
    pub path_prefix: String,
}
