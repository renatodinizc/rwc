use assert_cmd::Command;

#[test]
fn count_empty_file() {
  let mut cmd = Command::cargo_bin("rwc").unwrap();

  cmd.arg("tests/inputs/empty.txt");
  cmd.assert().success().stdout("  1 0 1 tests/inputs/empty.txt\n");
}