use crate::config::Template;

pub enum Error {
   ReplaceError(String),
}

pub fn create(templates: &Vec<Template>, name: &str, template_id: &str, subtitle: &str) -> Result<(), Error> {
    Ok(())
}
