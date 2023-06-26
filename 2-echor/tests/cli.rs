use std::error::Error;
use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// If no text is provided, show usage which matches string USAGE
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt" )
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt" )
}

#[test]
fn hello3() -> TestResult {
    run(&["Hello there", "-n"], "tests/expected/hello1.n.txt" )
}

#[test]
fn hello4() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt" )
}

// #[test]
// fn hello1() -> TestResult {
//     let expected = fs::read_to_string("tests/expected/hello1.txt")?;
//     let mut cmd = Command::cargo_bin("echor")?;
//     cmd.arg("Hello there").assert().success().stdout(expected);
//     Ok(())
// }
