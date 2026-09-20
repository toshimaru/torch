use std::fs::remove_file;
use std::process::{Command, Output};

fn run_command(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_torch"))
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
#[cfg(unix)]
fn test_fail_permission_denied() {
    let output = run_command(&["/etc/denied"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stdout, "");
    assert!(stderr.contains("Error creating a file(/etc/denied): Permission denied"));
}

#[test]
#[cfg(unix)]
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
    assert!(stderr.contains("Error creating a directory(test_fail_not_a_directory):"));
    remove_file(path).unwrap();
}
