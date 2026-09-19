use clap::Parser;
use filetime::{FileTime, set_file_times};
use std::fs::{OpenOptions, create_dir_all};
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

    for path in &args.paths {
        has_err |= !mkdir_touch(path);
    }

    if has_err {
        std::process::exit(1);
    }
}

fn mkdir_touch(path: &str) -> bool {
    let p = Path::new(path);

    // A trailing separator means the path itself is a directory,
    // otherwise only the parent directories need to be created
    let dir = if path.ends_with(std::path::is_separator) {
        Some(p)
    } else {
        p.parent().filter(|d| !d.as_os_str().is_empty())
    };

    // Create the directories
    if let Some(dir) = dir
        && let Err(e) = create_dir_all(dir)
    {
        eprintln!("Error creating a directory({}): {}", dir.display(), e);
        return false;
    }

    // Create a file
    if let Err(e) = touch(p) {
        eprintln!("Error creating a file({}): {}", path, e);
        return false;
    }

    true
}

fn touch(path: &Path) -> Result<()> {
    if !path.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(path)?;
    }
    let now = FileTime::now();
    set_file_times(path, now, now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::metadata;
    use std::fs::{File, remove_dir_all, remove_file};
    use tempfile::tempdir;

    #[test]
    fn test_touch_creates_file() {
        let path = Path::new("test_touch_creates_file");
        assert!(!Path::new(path).exists());
        assert!(touch(path).is_ok());
        assert!(Path::new(path).exists());
        remove_file(path).unwrap();
    }

    fn assert_touch_updates_timestamps(path: &Path) {
        let old = FileTime::from_unix_time(946684800, 0);
        set_file_times(path, old, old).unwrap();

        let before = FileTime::now();
        touch(path).unwrap();
        let after = FileTime::now();
        let metadata = metadata(path).unwrap();
        for timestamp in [
            FileTime::from_last_access_time(&metadata),
            FileTime::from_last_modification_time(&metadata),
        ] {
            assert!(timestamp > old);
            // Compare whole seconds to tolerate filesystem timestamp precision.
            assert!(timestamp.unix_seconds() >= before.unix_seconds());
            assert!(timestamp.unix_seconds() <= after.unix_seconds());
        }
    }

    #[test]
    fn test_touch_updates_timestamp() {
        let fixture = tempdir().unwrap();
        let path = fixture.path().join("file.txt");
        File::create(&path).unwrap();
        assert_touch_updates_timestamps(&path);
    }

    #[test]
    fn test_touch_updates_timestamp_for_directory() {
        let fixture = tempdir().unwrap();
        assert_touch_updates_timestamps(fixture.path());
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
    fn test_mkdir_touch_with_trailing_slash() {
        let dir = "test_mkdir_touch_with_trailing_slash";
        let path = format!("{}/", dir);
        remove_dir_all(dir).ok();
        assert!(mkdir_touch(&path));
        assert!(Path::new(dir).is_dir());
        remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_mkdir_touch_with_trailing_slash_nested() {
        let dir = "test_mkdir_touch_with_trailing_slash_nested";
        let path = format!("{}/{}", dir, "a/b/");
        remove_dir_all(dir).ok();
        assert!(mkdir_touch(&path));
        assert!(Path::new(&path).is_dir());
        remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_mkdir_touch_with_trailing_slash_already_exists() {
        let dir = "test_mkdir_touch_with_trailing_slash_already_exists";
        create_dir_all(dir).unwrap();
        assert!(mkdir_touch(&format!("{}/", dir)));
        assert!(Path::new(dir).is_dir());
        remove_dir_all(dir).unwrap();
    }

    #[test]
    #[cfg(windows)]
    fn test_mkdir_touch_with_trailing_backslash() {
        let dir = "test_mkdir_touch_with_trailing_backslash";
        let path = format!("{}\\", dir);
        remove_dir_all(dir).ok();
        assert!(mkdir_touch(&path));
        assert!(Path::new(dir).is_dir());
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
}
