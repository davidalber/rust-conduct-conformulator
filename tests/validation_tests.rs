extern crate code_of_conduct_conformulator;
extern crate reqwest;

use std::fs::File;
use std::io::Read;

const RUST_WWW_CODE_OF_CONDUCT: &str =
    "https://raw.githubusercontent.com/rust-lang/rust-www/master/en-US/conduct.md";

fn fetch(url: &str) -> String {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());
    resp.text().unwrap()
}

fn read_expected(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

#[test]
fn validate_base_file() {
    let base = fetch(RUST_WWW_CODE_OF_CONDUCT);
    let expected_base = read_expected("tests/expected/base.md");

    assert!(
        expected_base == base,
        format!(
            "Rust WWW code of conduct ({}) does not match expected value",
            RUST_WWW_CODE_OF_CONDUCT
        )
    );
}
