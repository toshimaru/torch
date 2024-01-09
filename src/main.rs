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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;
    use std::fs::metadata;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_touch_creates_file() {
        let test_path = format!("{}.txt", std::module_path!());
        assert!(!Path::new(&test_path).exists());
        touch(&test_path).unwrap();
        assert!(Path::new(&test_path).exists());
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_touch_updates_timestamp() {
        let test_path = format!("{}.txt", std::module_path!());
        fs::File::create(&test_path).unwrap();
        thread::sleep(Duration::from_secs(1));
        touch(&test_path).unwrap();
        let metadata = metadata(&test_path).unwrap();
        let modified_time = FileTime::from_last_modification_time(&metadata);
        assert_eq!(modified_time.unix_seconds(), FileTime::now().unix_seconds());
        fs::remove_file(test_path).unwrap();
    }
}
