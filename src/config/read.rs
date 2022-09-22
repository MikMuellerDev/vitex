use super::{errors::Result, Config};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn file_path() -> Option<String> {
    match env::var("HOME") {
        Ok(home) => {
            if let Ok(xdg_home) = env::var("XDG_CONFIG_HOME") {
                Some(format!("{}/vitex", xdg_home))
            } else {
                Some(format!("{}/.config/vitex", home))
            }
        }
        Err(_) => None,
    }
}

pub fn read_config(base_path: &Path) -> Result<Config> {
    let config_path = base_path.join("config.toml");
    match config_path.exists() {
        true => Ok(toml::from_str::<Config>(&fs::read_to_string(config_path)?)?.validate()?),
        false => {
            println!("");
            fs::create_dir_all(
                config_path
                    .parent()
                    .expect("Config file path is expected to have a parent"),
            )?;
            let mut file = File::create(config_path)?;
            file.write_all(
                &toml::to_vec(&Config::default())
                    .expect("The config struct must always be encodable"),
            )?;
            Ok(Config::default())
        }
    }
}
