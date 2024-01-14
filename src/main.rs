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
    let mut has_err = false;

    for path in args.paths {
        if !mkdir_touch(&path) {
            has_err = true;
        }
    }

    if has_err {
        std::process::exit(1);
    }
}

fn mkdir_touch(path: &str) -> bool {
    let p = Path::new(&path);

    // Create directory if it contains directories
    if path.contains('/') {
        if let Some(dir) = p.parent() {
            match mkdir(dir) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error creating a directory({}): {}", dir.display(), e);
                    return false;
                }
            }
        }
    }

    // Create file
    match touch(p) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error creating a file({}): {}", path, e);
            return false;
        }
    }

    true
}

fn mkdir(dir: &Path) -> Result<()> {
    if !dir.exists() {
        create_dir_all(dir)?;
    }
    Ok(())
}

fn touch(path: &Path) -> Result<()> {
    if !path.exists() {
        OpenOptions::new().create(true).write(true).open(path)?;
    }
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
        let path = Path::new("test_touch_creates_file");
        assert!(!Path::new(path).exists());
        assert!(touch(path).is_ok());
        assert!(Path::new(path).exists());
        remove_file(path).unwrap();
    }

    #[test]
    fn test_touch_updates_timestamp() {
        let path = Path::new("test_touch_updates_timestamp");
        File::create(path).unwrap();
        thread::sleep(Duration::from_secs(1));
        assert!(touch(path).is_ok());
        let metadata = metadata(path).unwrap();
        let modified_time = FileTime::from_last_modification_time(&metadata);
        assert_eq!(modified_time.unix_seconds(), FileTime::now().unix_seconds());
        remove_file(path).unwrap();
    }

    #[test]
    fn test_touch_updates_timestamp_for_directory() {
        let dir = Path::new("test_touch_updates_timestamp_for_directory");
        create_dir_all(dir).unwrap();
        thread::sleep(Duration::from_secs(1));
        assert!(touch(dir).is_ok());
        let metadata = metadata(dir).unwrap();
        let modified_time = FileTime::from_last_modification_time(&metadata);
        assert_eq!(modified_time.unix_seconds(), FileTime::now().unix_seconds());
        remove_dir_all(dir).unwrap();
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

    #[test]
    fn test_mkdir_touch_with_directory1() {
        let dir = "test_mkdir_touch_with_directory1";
        let path = format!("{}/{}", dir, "a.txt");
        remove_dir_all(dir).ok();
        assert!(mkdir_touch(&path));
        assert!(Path::new(&path).exists());
        remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_mkdir_touch_with_directory2() {
        let dir = "test_mkdir_touch_with_directory2";
        let path = format!("{}/{}", dir, "a/b/c.txt");
        remove_dir_all(dir).ok();
        assert!(mkdir_touch(&path));
        assert!(Path::new(&path).exists());
        remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_mkdir_touch_without_directory() {
        let path = "test_mkdir_touch_without_directory";
        remove_file(path).ok();
        assert!(mkdir_touch(path));
        assert!(Path::new(path).exists());
        remove_file(path).unwrap();
    }

    #[test]
    fn test_mkdir_touch_error() {
        let path = "test_mkdir_touch_error";
        File::create(path).unwrap();
        let create_path = format!("{}/{}", path, "a.txt");
        assert!(!mkdir_touch(&create_path));
        assert!(!Path::new(&create_path).exists());
        remove_file(path).unwrap();
    }

    mod integration_tests {
        use std::fs::remove_file;
        use std::process::{Command, Output};

        fn run_command(args: &[&str]) -> Output {
            Command::new("cargo")
                .args(["run", "--quiet"])
                .args(args)
                .output()
                .expect("Command failed")
        }

        #[test]
        fn test_success_output() {
            let output = run_command(&[]);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert_eq!(stdout, "");
            assert_eq!(stderr, "");
        }

        #[test]
        fn test_fail_permission_denied() {
            let output = run_command(&["/etc/denied"]);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert_eq!(stdout, "");
            assert!(stderr.contains("Error creating a file(/etc/denied): Permission denied"));
        }

        #[test]
        fn test_fail_operation_not_permitted() {
            let output = run_command(&["/etc/passwd"]);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert_eq!(stdout, "");
            assert!(stderr.contains("Error creating a file(/etc/passwd): Operation not permitted"));
        }

        #[test]
        fn test_fail_not_a_directory() {
            let path = "test_fail_not_a_directory";
            let output = run_command(&[path, format!("{}/{}", path, "test.txt").as_str()]);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert_eq!(stdout, "");
            assert!(stderr.contains(
                "Error creating a file(test_fail_not_a_directory/test.txt): Not a directory"
            ));
            remove_file(path).unwrap();
        }
    }
}
