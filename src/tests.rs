use super::execute;

const EXAMPLE_DIR: &'static str = "./example";

fn _vec_str(args: Vec<&str>) -> Vec<String> {
    args.iter().map(|s| s.to_string()).collect()
}

/// basic test for a simple script.
/// the output should be the path to the script itself.
#[test]
fn test_simple_script() {
    assert_eq!(
        execute(_vec_str(vec!["tome", EXAMPLE_DIR, "file_example"])),
        Ok(format!("{}/file_example", EXAMPLE_DIR))
    );
}

#[test]
fn test_simple_script_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            EXAMPLE_DIR,
            "file_example",
            "--complete"
        ])),
        Ok(String::from("file autocomplete example"))
    );
}

/// basic test for a script that should be sourced
#[test]
fn test_source() {
    assert_eq!(
        execute(_vec_str(vec!["tome", EXAMPLE_DIR, "source_example",])),
        Ok(format!("source {}/source_example \"\"", EXAMPLE_DIR))
    );
}

#[test]
fn test_source_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            EXAMPLE_DIR,
            "source_example",
            "--complete"
        ])),
        Ok(String::from("foo baz\n"))
    );
}

/// if completion is requested on a directory,
/// return the list of file and directories in there.
#[test]
fn test_directory_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            EXAMPLE_DIR,
            "dir_example",
            "--complete"
        ])),
        Ok("bar foo".to_string())
    );
}

/// the root directory should also be completed
#[test]
fn test_root_directory_completion() {
    assert_eq!(
        execute(_vec_str(vec!["tome", EXAMPLE_DIR, "--complete"])),
        Ok("source_example file_example use-arg dir_example".to_string())
    );
}

/// if completion is requested on a directory,
/// return the list of file and directories in there.
#[test]
fn test_script_in_directory() {
    assert_eq!(
        execute(_vec_str(vec!["tome", EXAMPLE_DIR, "dir_example", "foo"])),
        Ok(format!("{}/dir_example/foo", EXAMPLE_DIR))
    );
}

/// if completion is requested on a directory,
/// return the list of file and directories in there.
#[test]
fn test_use_arg() {
    assert_eq!(
        execute(_vec_str(vec!["tome", EXAMPLE_DIR, "use-arg"])),
        Ok(format!("source {}/use-arg \"\"", EXAMPLE_DIR))
    );
}
