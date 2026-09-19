use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::tempdir;

fn run_command(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_torch"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to run torch")
}

#[test]
fn test_success_output() {
    let fixture = tempdir().unwrap();
    for args in [&[][..], &["a/b/file.txt", "directory/"][..]] {
        let output = run_command(fixture.path(), args);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }
    assert!(fixture.path().join("a/b/file.txt").is_file());
    assert!(fixture.path().join("directory").is_dir());
}

#[test]
fn test_existing_file_contents_survive() {
    let fixture = tempdir().unwrap();
    let path = fixture.path().join("existing.txt");
    let contents = b"keep this content\n\0\xff";
    fs::write(&path, contents).unwrap();

    let output = run_command(fixture.path(), &["existing.txt", "existing.txt"]);

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert_eq!(fs::read(path).unwrap(), contents);
}

#[test]
fn test_directory_creation_failure_exits_one_and_continues() {
    for args in [
        vec!["blocked/child.txt", "later/nested/file.txt"],
        vec!["earlier.txt", "blocked/child.txt", "later/nested/file.txt"],
        vec!["earlier.txt", "blocked/child.txt"],
        vec![
            "blocked/child.txt",
            "blocked/other.txt",
            "later/nested/file.txt",
        ],
    ] {
        let fixture = tempdir().unwrap();
        fs::write(fixture.path().join("blocked"), b"blocker").unwrap();

        let output = run_command(fixture.path(), &args);

        assert_eq!(output.status.code(), Some(1), "args: {args:?}");
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Error creating a directory(blocked):"));
        for path in args {
            if path.starts_with("blocked/") {
                assert!(!fixture.path().join(path).exists());
            } else {
                assert!(fixture.path().join(path).is_file(), "missing {path}");
            }
        }
        assert_eq!(
            fs::read(fixture.path().join("blocked")).unwrap(),
            b"blocker"
        );
    }
}

#[test]
#[cfg(unix)]
fn test_fail_permission_denied() {
    let fixture = tempdir().unwrap();
    let output = run_command(fixture.path(), &["/etc/denied", "later.txt"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Error creating a file(/etc/denied): Permission denied"),
        "unexpected stderr: {stderr}"
    );
    assert!(fixture.path().join("later.txt").is_file());
}

#[test]
#[cfg(unix)]
fn test_fail_operation_not_permitted() {
    let fixture = tempdir().unwrap();
    let output = run_command(fixture.path(), &["/etc/passwd", "later.txt"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Error creating a file(/etc/passwd): Operation not permitted"),
        "unexpected stderr: {stderr}"
    );
    assert!(fixture.path().join("later.txt").is_file());
}

#[test]
fn test_file_creation_failure_exits_one_and_continues() {
    let fixture = tempdir().unwrap();
    // An empty path fails file creation without depending on OS permissions.
    let output = run_command(fixture.path(), &["", "later.txt"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Error creating a file():"));
    assert!(fixture.path().join("later.txt").is_file());
}
