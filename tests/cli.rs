use std::fs::{remove_dir_all, remove_file};
use std::path::Path;
use std::process::{Command, Output};

fn run_command(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_torch"))
        .args(args)
        .output()
        .expect("failed to run torch")
}

#[test]
fn test_success_output() {
    let dir = "test_success_output";
    let file = format!("{dir}/nested/file.txt");
    let directory = format!("{dir}/directory/");

    for args in [&[][..], &[file.as_str(), directory.as_str()][..]] {
        let output = run_command(args);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }

    assert!(Path::new(&file).is_file());
    assert!(Path::new(&directory).is_dir());
    remove_dir_all(dir).unwrap();
}

#[test]
#[cfg(unix)]
fn test_fail_permission_denied() {
    let output = run_command(&["/etc/denied"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error creating a file(/etc/denied): Permission denied"));
}

#[test]
#[cfg(unix)]
fn test_fail_operation_not_permitted() {
    let output = run_command(&["/etc/passwd"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error creating a file(/etc/passwd): Operation not permitted"));
}

#[test]
fn test_fail_not_a_directory() {
    let path = "test_fail_not_a_directory";
    let output = run_command(&[path, format!("{}/{}", path, "test.txt").as_str()]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error creating a directory(test_fail_not_a_directory):"));
    assert!(Path::new(path).is_file());
    remove_file(path).unwrap();
}
