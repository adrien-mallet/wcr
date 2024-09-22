use std::fs;

use assert_cmd::Command;

#[test]
fn simple_test() {
    run_test(
        &["tests/resources/simple_test.input"],
        "tests/resources/simple_test.output",
    )
}

fn run_test(args: &[&str], expected_resource: &str) {
    let mut bin = Command::cargo_bin("wcr").unwrap();
    let output = bin
        .args(args)
        .output()
        .expect("get binary output failed")
        .stdout;
    assert_eq!(
        fs::read_to_string(expected_resource).expect("Error reading file resource"),
        String::from_utf8(output).expect("Error when converting Utf8 string")
    );
}
