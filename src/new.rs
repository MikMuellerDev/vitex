use std::io;

pub fn new(name: &str, template_id: Option<String>) -> io::Result<()> {
    println!(
        "Creating {name} with template {}",
        template_id.unwrap_or("default".to_string())
    );
    Ok(())
}
