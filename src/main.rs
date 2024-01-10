use clap::Parser;
use filetime::{set_file_times, FileTime};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Result;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    for file in args.files {
        // Create directory if it contains directories
        if file.contains('/') {
            let path = Path::new(&file);
            if let Some(parent) = path.parent() {
                match create_dir_all(parent) {
                    Ok(_) => println!("Directory({}) created successfully", parent.display()),
                    Err(e) => println!("Error creating directory({}): {}", parent.display(), e),
                }
            }
        }

        // Create file
        match touch(file.as_str()) {
            Ok(_) => println!("File created successfully"),
            Err(e) => println!("Error creating file({}): {}", file, e),
        }
    }
}

fn touch(path: &str) -> Result<()> {
    OpenOptions::new().create(true).write(true).open(path)?;
    let now: FileTime = FileTime::now();
    set_file_times(path, now, now)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::metadata;
    use std::path::Path;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_touch_creates_file() {
        let test_path = format!("{}.txt", std::module_path!());
        assert!(!Path::new(&test_path).exists());
        assert!(touch(&test_path).is_ok());
        assert!(Path::new(&test_path).exists());
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_touch_updates_timestamp() {
        let test_path = format!("{}.txt", std::module_path!());
        fs::File::create(&test_path).unwrap();
        thread::sleep(Duration::from_secs(1));
        assert!(touch(&test_path).is_ok());
        let metadata = metadata(&test_path).unwrap();
        let modified_time = FileTime::from_last_modification_time(&metadata);
        assert_eq!(modified_time.unix_seconds(), FileTime::now().unix_seconds());
        fs::remove_file(test_path).unwrap();
    }
}
