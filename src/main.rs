use std::fs::{OpenOptions, create_dir_all};
use std::io::Result;
use filetime::{FileTime, set_file_times};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    for file in args.files {
        match touch(file.as_str()) {
            Ok(_) => println!("File created successfully"),
            Err(e) => println!("Error creating file: {}", e),
        }
    }

    match create_dir_all("my_directory") {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {}", e),
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
