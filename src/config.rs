use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IO(io::Error),
    TomlDecode(toml::de::Error),
    TomlEncode(toml::ser::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::TomlDecode(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::TomlEncode(err)
    }
}

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
    pub git_repository: String,
}

impl Default for Template {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            local_path: String::new(),
            git_repository: "git@github.com/foo/bar".to_string(),
        }
    }
}

pub fn file_path() -> Option<String> {
    match env::var("HOME") {
        Ok(home) => {
            if let Ok(xdg_home) = env::var("XDG_CONFIG_HOME") {
                Some(format!("{}/vitex/config.toml", xdg_home))
            } else {
                Some(format!("{}/.config/vitex/config.toml", home))
            }
        }
        Err(_) => None,
    }
}

pub fn read_config(path: &str) -> Result<Config> {
    let path = Path::new(path);

    match path.exists() {
        true => Ok(toml::from_str(&fs::read_to_string(path)?)?),
        false => {
            println!("");
            fs::create_dir_all(
                path.parent()
                    .expect("Config file path is expected to have a parent"),
            )?;
            let mut file = File::create(path)?;
            file.write_all(&toml::to_vec(&Config::default())?)?;
            Ok(Config::default())
        }
    }
}
