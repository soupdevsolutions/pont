pub fn get_current_directory() -> Result<String, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let current_dir_name = current_dir
        .clone()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    Ok(current_dir_name)
}
