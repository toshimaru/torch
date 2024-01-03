use std::fs;
use std::io::Result;

fn main() {
    match fs::create_dir_all("my_directory") {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {}", e),
    }
    match touch("example.txt") {
        Ok(_) => println!("File created successfully"),
        Err(e) => println!("Error creating file: {}", e),
    }
}

fn touch(path: &str) -> Result<()> {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;
    Ok(())
}
