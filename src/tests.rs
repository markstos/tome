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
        execute(_vec_str(vec![EXAMPLE_DIR, "foobar"])),
        Ok(format!("{}/foobar", EXAMPLE_DIR))
    );
}

#[test]
fn test_simple_script_completion() {
    assert_eq!(
        execute(_vec_str(vec![EXAMPLE_DIR, "foobar", "--complete"])),
        Ok(String::from("file autocomplete example"))
    );
}

/// if completion is requested on a directory,
/// return the list of file and directories in there.
#[test]
fn test_directory_completion() {
    assert_eq!(
        execute(_vec_str(vec![EXAMPLE_DIR, "dir_example", "--complete"])),
        Ok("bar foo".to_string())
    );
}

/// if completion is requested on a directory,
/// return the list of file and directories in there.
#[test]
fn test_script_in_directory() {
    assert_eq!(
        execute(_vec_str(vec![EXAMPLE_DIR, "dir_example", "foo"])),
        Ok(format!("{}/dir_example/foo", EXAMPLE_DIR))
    );
}
