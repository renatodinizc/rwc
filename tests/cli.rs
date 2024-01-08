use assert_cmd::Command;

#[test]
fn count_empty_file() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/empty.txt");

    cmd.assert()
        .success()
        .stdout("1 0 1 tests/inputs/empty.txt\n");
}

#[test]
fn count_non_readable_file() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/non_readable.txt");

    cmd.assert()
        .success()
        .stderr("wc: tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
}

#[test]
fn count_phrase_file() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/phrase.txt");

    cmd.assert()
        .success()
        .stdout("1 16 90 tests/inputs/phrase.txt\n");
}

#[test]
fn count_emojis_file() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/emojis.txt");

    cmd.assert()
        .success()
        .stdout("1 1 33 tests/inputs/emojis.txt\n");
}

#[test]
fn count_tao_file() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/tao.txt");

    cmd.assert()
        .success()
        .stdout("6 57 299 tests/inputs/tao.txt\n");
}

#[test]
fn count_all_files() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/empty.txt");
    cmd.arg("tests/inputs/non_readable.txt");
    cmd.arg("tests/inputs/phrase.txt");
    cmd.arg("tests/inputs/tao.txt");
    cmd.arg("tests/inputs/emojis.txt");

    cmd.assert()
        .success()
        .stderr("wc: tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
    cmd.assert().success().stdout(
        "1 0 1 tests/inputs/empty.txt
1 16 90 tests/inputs/phrase.txt
6 57 299 tests/inputs/tao.txt
1 1 33 tests/inputs/emojis.txt
9 74 423 total\n",
    );
}

#[test]
fn count_from_stdin() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.write_stdin(
        "Peter Piper picked a peck of pickled peppers.
A peck of pickled peppers Peter Piper picked.
If Peter Piper picked a peck of pickled peppers,
Where’s the peck of pickled peppers Peter Piper picked?
",
    );

    cmd.assert().success().stdout("4 34 199\n");
}

#[test]
fn count_file_takes_priority_over_stdin() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.write_stdin(
        "Peter Piper picked a peck of pickled peppers.
A peck of pickled peppers Peter Piper picked.
If Peter Piper picked a peck of pickled peppers,
Where’s the peck of pickled peppers Peter Piper picked?
",
    );

    cmd.arg("tests/inputs/tao.txt");

    cmd.assert()
        .success()
        .stdout("6 57 299 tests/inputs/tao.txt\n");
}

#[test]
fn count_empty_file_takes_priority_over_stdin() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.write_stdin(
        "Peter Piper picked a peck of pickled peppers.
A peck of pickled peppers Peter Piper picked.
If Peter Piper picked a peck of pickled peppers,
Where’s the peck of pickled peppers Peter Piper picked?
",
    );

    cmd.arg("tests/inputs/empty.txt");

    cmd.assert()
        .success()
        .stdout("1 0 1 tests/inputs/empty.txt\n");
}

#[test]
fn count_all_files_with_l_option() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/empty.txt");
    cmd.arg("tests/inputs/non_readable.txt");
    cmd.arg("tests/inputs/phrase.txt");
    cmd.arg("tests/inputs/tao.txt");
    cmd.arg("tests/inputs/emojis.txt");
    cmd.arg("-l");

    cmd.assert()
        .success()
        .stderr("wc: tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
    cmd.assert().success().stdout(
        "1 tests/inputs/empty.txt
1 tests/inputs/phrase.txt
6 tests/inputs/tao.txt
1 tests/inputs/emojis.txt
9 total\n",
    );
}

#[test]
fn count_all_files_with_w_option() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/empty.txt");
    cmd.arg("tests/inputs/non_readable.txt");
    cmd.arg("tests/inputs/phrase.txt");
    cmd.arg("tests/inputs/tao.txt");
    cmd.arg("tests/inputs/emojis.txt");
    cmd.arg("-w");

    cmd.assert()
        .success()
        .stderr("wc: tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
    cmd.assert().success().stdout(
        "0 tests/inputs/empty.txt
16 tests/inputs/phrase.txt
57 tests/inputs/tao.txt
1 tests/inputs/emojis.txt
74 total\n",
    );
}

#[test]
fn count_all_files_with_c_option() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/empty.txt");
    cmd.arg("tests/inputs/non_readable.txt");
    cmd.arg("tests/inputs/phrase.txt");
    cmd.arg("tests/inputs/tao.txt");
    cmd.arg("tests/inputs/emojis.txt");
    cmd.arg("-c");

    cmd.assert()
        .success()
        .stderr("wc: tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
    cmd.assert().success().stdout(
        "1 tests/inputs/empty.txt
90 tests/inputs/phrase.txt
299 tests/inputs/tao.txt
33 tests/inputs/emojis.txt
423 total\n",
    );
}

#[test]
fn count_all_files_with_m_option() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/empty.txt");
    cmd.arg("tests/inputs/non_readable.txt");
    cmd.arg("tests/inputs/phrase.txt");
    cmd.arg("tests/inputs/tao.txt");
    cmd.arg("tests/inputs/emojis.txt");
    cmd.arg("-m");

    cmd.assert()
        .success()
        .stderr("wc: tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
    cmd.assert().success().stdout(
        "1 tests/inputs/empty.txt
90 tests/inputs/phrase.txt
299 tests/inputs/tao.txt
9 tests/inputs/emojis.txt
399 total\n",
    );
}

#[test]
fn count_all_files_with_mw_option() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();

    cmd.arg("tests/inputs/empty.txt");
    cmd.arg("tests/inputs/non_readable.txt");
    cmd.arg("tests/inputs/phrase.txt");
    cmd.arg("tests/inputs/tao.txt");
    cmd.arg("tests/inputs/emojis.txt");
    cmd.arg("-mw");

    cmd.assert()
        .success()
        .stderr("wc: tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
    cmd.assert().success().stdout(
        "0 1 tests/inputs/empty.txt
16 90 tests/inputs/phrase.txt
57 299 tests/inputs/tao.txt
1 9 tests/inputs/emojis.txt
74 399 total\n",
    );
}
