use std::{fs, io, path::Path};

pub fn create_templates_directory(base_path: &Path) -> Result<(), io::Error> {
    let local_templates_path = base_path.join("templates").join("local");
    match local_templates_path.exists() {
        true => {
            todo!()
        }
        false => fs::create_dir_all(local_templates_path)?,
    }
    let cloned_templates_path = base_path.join("templates").join(".cloned");
    match cloned_templates_path.exists() {
        true => {
            todo!()
        }
        false => fs::create_dir_all(cloned_templates_path)?,
    }
    Ok(())
}

