use std::fs::{OpenOptions, create_dir_all};
use std::io::Result;
use filetime::{FileTime, set_file_times};

fn main() {
    match create_dir_all("my_directory") {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {}", e),
    }
    match touch("example.txt") {
        Ok(_) => println!("File created successfully"),
        Err(e) => println!("Error creating file: {}", e),
    }
}

fn touch(path: &str) -> Result<()> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;
    let now: FileTime = FileTime::now();
    set_file_times(path, now, now)?;
    Ok(())
}
