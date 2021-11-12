use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_is_callable() {
    let mut cmd = Command::cargo_bin("hip-validator").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn fail_on_missing_fm_block() {
    // try to initialize a holium repository
    let mut cmd = Command::cargo_bin("hip-validator").unwrap();
    let assert = cmd
        .current_dir(project_root::get_project_root().unwrap())
        .arg("./tests/assets/missing_fm.md")
        .assert();
    // check error message
    assert.failure().stderr(predicate::str::contains(
        "markdown file should contain a YAML front matter block",
    ));
}

#[test]
fn fail_on_missing_field() {
    // try to initialize a holium repository
    let mut cmd = Command::cargo_bin("hip-validator").unwrap();
    let assert = cmd
        .current_dir(project_root::get_project_root().unwrap())
        .arg("./tests/assets/missing_field.md")
        .assert();
    // check error message
    assert
        .failure()
        .stderr(predicate::str::contains("missing field `author`"));
}

#[test]
fn fail_on_wrong_date_format() {
    // try to initialize a holium repository
    let mut cmd = Command::cargo_bin("hip-validator").unwrap();
    let assert = cmd
        .current_dir(project_root::get_project_root().unwrap())
        .arg("./tests/assets/wrong_date_format.md")
        .assert();
    // check error message
    assert
        .failure()
        .stderr(predicate::str::contains("date is not iso8601 compatible"));
}

#[test]
fn pass_with_correct_fm_block() {
    // try to initialize a holium repository
    let mut cmd = Command::cargo_bin("hip-validator").unwrap();
    let assert = cmd
        .current_dir(project_root::get_project_root().unwrap())
        .arg("./tests/assets/pass.md")
        .assert();
    // check error message
    assert.success();
}
