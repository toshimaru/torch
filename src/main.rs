use clap::Parser;
use filetime::{set_file_times, FileTime};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Result;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for path in args.paths {
        // Create directory if it contains directories
        if path.contains('/') {
            let path = Path::new(&path);
            if let Some(dir) = path.parent() {
                match mkdir(dir) {
                    Ok(_) => println!("Directory({}) created successfully", dir.display()),
                    Err(e) => {
                        println!("Error creating directory({}): {}", dir.display(), e)
                    }
                }
            }
        }

        // Create file
        match touch(path.as_str()) {
            Ok(_) => println!("File created successfully"),
            Err(e) => println!("Error creating file({}): {}", path, e),
        }
    }
}

fn mkdir(dir: &Path) -> Result<()> {
    create_dir_all(dir)?;
    Ok(())
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
    use std::fs::metadata;
    use std::fs::{remove_dir_all, remove_file, File};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_touch_creates_file() {
        let test_path = "test_touch_creates_file";
        assert!(!Path::new(test_path).exists());
        assert!(touch(test_path).is_ok());
        assert!(Path::new(test_path).exists());
        remove_file(test_path).unwrap();
    }

    #[test]
    fn test_touch_updates_timestamp() {
        let test_path = "test_touch_updates_timestamp";
        File::create(test_path).unwrap();
        thread::sleep(Duration::from_secs(1));
        assert!(touch(test_path).is_ok());
        let metadata = metadata(test_path).unwrap();
        let modified_time = FileTime::from_last_modification_time(&metadata);
        assert_eq!(modified_time.unix_seconds(), FileTime::now().unix_seconds());
        remove_file(test_path).unwrap();
    }

    #[test]
    fn test_mkdir_success() {
        let dir = Path::new("test_mkdir_success");
        assert!(!dir.exists());
        assert!(mkdir(dir).is_ok());
        assert!(dir.exists());
        remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_mkdir_already_exists() {
        let existing_dir = Path::new("test_mkdir_already_exists");
        create_dir_all(existing_dir).unwrap();
        assert!(mkdir(existing_dir).is_ok());
        remove_dir_all(existing_dir).unwrap();
    }
}
